<template>
    <div class="tab-title">
        <h2>Import</h2>
        <p>Click on a page to import it into Rift Buddy</p>
    </div>
    <div class="pages" v-if="connected">
        <div class="page" v-for="page of pages" @click="importPage(page)" :key="page.id">
            <div class="main-rune">
                <img :src="runeImg(findRune(page.primaryStyleId))" />
            </div>
            <div class="main-rune second">
                <img :src="runeImg(findRune(page.subStyleId))" />
            </div>
            <div class="page-name">{{ page.name }}</div>
            <div class="ff"></div>
            <div class="config">
                <div class="other-rune" v-for="perk in page.selectedPerkIds.slice(0, -3)">
                    <img :src="runeImg(findRune(perk))" />
                </div>
                <div class="stat-mod" v-for="perk in page.selectedPerkIds.slice(-3)">
                    <img :src="runeImg(findRune(perk))" />
                </div>
            </div>
        </div>
    </div>
    <div class="connecting" v-else>
        <div class="loading primary"></div>
        <p>Searching for League of Legends...</p>
    </div>
</template>

<script>
import axios from "axios";
import { v4 as uuid } from "uuid";
import { invoke } from "@tauri-apps/api/tauri";

export default {
    inject: ["connected"],
    name: "ImportPage",
    data: () => {
        return {
            pages: [],
            runes: [],
            stat_mods: [
                { id: 5008, desc: "+9 Adaptive Force", icon: "perk-images/StatMods/StatModsAdaptiveForceIcon.png" },
                { id: 5005, desc: "+10 Attack Speed", icon: "perk-images/StatMods/StatModsAttackSpeedIcon.png" },
                { id: 5007, desc: "+8 Ability Haste", icon: "perk-images/StatMods/StatModsCDRScalingIcon.png" },
                { id: 5002, desc: "+6 Armor", icon: "perk-images/StatMods/StatModsArmorIcon.png" },
                { id: 5003, desc: "+8 Magic Resist", icon: "perk-images/StatMods/StatModsMagicResIcon.MagicResist_Fix.png" },
                { id: 5001, desc: "+15-140 Health (based on level)", icon: "perk-images/StatMods/StatModsHealthScalingIcon.png" },
            ],
            ddversion: null,
            timeout: null,
        };
    },
    computed: {
        flattened() {
            const flattened = [];
            for (const page of this.runes) {
                for (const perk of page.slots) {
                    for (const rune of perk.runes) {
                        flattened.push(rune);
                    }
                }
                const copy = JSON.parse(JSON.stringify(page));
                delete copy.slots;
                flattened.push(copy);
            }
            for (const stat of this.stat_mods) {
                flattened.push(stat);
            }
            return flattened;
        },
    },
    mounted() {
        this.initialize();
    },
    methods: {
        async initialize() {
            if (!this.connected) {
                clearTimeout(this.timeout);
                this.timeout = setTimeout(this.initialize, 1000);
                return;
            }

            await axios.get("https://ddragon.leagueoflegends.com/api/versions.json").then((res) => {
                this.ddversion = res.data[0];
            });

            await axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.ddversion}/data/en_US/runesReforged.json`).then((res) => {
                this.runes = res.data;
            });

            this.pages = await invoke("lcu", { endpoint: "/lol-perks/v1/pages", method: "GET" }).catch(console.error);
            // console.log(this.pages);
            // console.log(this.flattened);
        },
        findRune(rune_id) {
            return this.flattened.find((rune) => rune.id === rune_id);
        },
        runeImg(rune) {
            return `./${rune.icon.replace("perk-images", "images/perks")}`;
        },
        async importPage(page) {
            const id = uuid();
            invoke("save_page", { id, page: { ...page, internal_id: id } })
                .then((res) => {
                    this.$bus.emit("app.toast", { message: "Page imported!" });
                })
                .catch(() => {
                    this.$bus.emit("app.toast", { message: "Failed to import page", type: "error" });
                });
        },
    },
};
</script>

<style scoped lang="less">
@import "@/assets/css/main.less";
// @import "@/assets/css/runes.less";

.connecting {
    height: calc(100% - 55px);
    p {
        margin: 0;
        font-size: 12px;
        line-height: 16px;
        color: #aaa;
    }
}

.pages {
    display: flex;
    flex-direction: column;
    flex: 1;
}
.page {
    flex: 1;
    display: flex;
    align-items: center;
    margin-bottom: 7px;
    // background-color: #1e1e1e;
    background-color: #202020;
    padding: 4px;
    border-radius: 5px;
    cursor: pointer;
    border: #1a1a1a solid 2px;

    &:hover {
        background-color: fade(#202020, 50%);
    }

    &:last-child {
        margin-bottom: 0;
    }

    &:hover {
        .other-rune,
        .stat-mod {
            margin-right: 2px;
        }
    }
}

.page-name {
    margin-left: 3px;
    font-size: 15px;
    line-height: 18px;
    font-weight: 600;
    white-space: nowrap;
}

.main-rune {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 28px;
    width: 28px;
    min-height: 28px;
    min-width: 28px;
    margin-right: 4px;
    border: #111 solid 2px;
    background-color: #191919;
    // padding: 4px;
    border-radius: 100%;

    &.second {
        margin-left: -15px;
    }

    img {
        width: 16px;
        height: 16px;
    }
}

.config {
    display: flex;
    flex-direction: row;
    align-items: center;
}

.other-rune,
.stat-mod {
    margin-right: 2px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: #111 solid 2px;
    background-color: #191919;
    border-radius: 99px;
    margin-right: -10px;
    height: 24px;
    width: 24px;
    min-height: 24px;
    min-width: 24px;
    transition: margin-right 0.1s ease-in-out;

    img {
        width: 20px;
    }

    &:last-child {
        margin-right: 0;
    }
}

.stat-mod {
    height: 20px;
    width: 20px;
    min-height: 20px;
    min-width: 20px;

    img {
        width: 16px;
    }
}
</style>
