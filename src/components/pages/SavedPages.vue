<template>
    <EditPage v-if="editing" :page="editing" @close="editClose()" />
    <template v-else>
        <div class="tab-title">
            <h2>Saved Pages</h2>
            <p>Click on a page to export and load it into League</p>
        </div>
        <div class="pages">
            <div class="page" v-for="_page of pages" @click="loadPage(_page[1])" :key="_page[0]" :set="(page = _page[1])">
                <div class="main-rune" :title="findRune(page.primaryStyleId).name">
                    <img :src="runeImg(findRune(page.primaryStyleId))" />
                </div>
                <div class="main-rune second" :title="findRune(page.subStyleId).name">
                    <img :src="runeImg(findRune(page.subStyleId))" />
                </div>
                <div class="page-name">{{ page.name }}</div>
                <div class="ff"></div>
                <div class="config">
                    <div class="other-rune" v-for="perk in page.selectedPerkIds.slice(0, -3)" :title="findRune(perk).name">
                        <img :src="runeImg(findRune(perk))" />
                    </div>
                    <div class="stat-mod" v-for="perk in page.selectedPerkIds.slice(-3)" :title="findRune(perk).desc">
                        <img :src="runeImg(findRune(perk))" />
                    </div>
                </div>
                <div class="options flex flex-align flex-row">
                    <div class="button icon" @click.stop="editPage(_page[1])">
                        <EditPencil color="white" width="16px" height="16px" stroke-width="2" />
                    </div>
                    <div class="button red icon" @click.stop="deletePage(_page[1])">
                        <Trash color="white" width="16px" height="16px" stroke-width="2" />
                    </div>
                </div>
            </div>
        </div>
    </template>
</template>

<script>
import axios from "axios";
import { v4 as uuid } from "uuid";
import { invoke } from "@tauri-apps/api/tauri";
import { Trash, EditPencil } from "@iconoir/vue";
import EditPage from "@/components/pages/EditPage.vue";

export default {
    name: "SavedPages",
    components: {
        Trash,
        EditPencil,
        EditPage,
    },
    data: () => {
        return {
            editing: null,
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
    async mounted() {
        await axios.get("https://ddragon.leagueoflegends.com/api/versions.json").then((res) => {
            this.ddversion = res.data[0];
        });

        await axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.ddversion}/data/en_US/runesReforged.json`).then((res) => {
            this.runes = res.data;
        });

        this.pages = await invoke("get_saved_pages");
        // console.log(this.runes);
        // console.log(this.pages);
        // console.log(this.flattened);
    },
    methods: {
        async editClose() {
            this.editing = null;
            this.pages = await invoke("get_saved_pages");
        },
        findRune(rune_id) {
            return this.flattened.find((rune) => rune.id === rune_id);
        },
        runeImg(rune) {
            return `./${rune.icon.replace("perk-images", "images/perks")}`;
        },
        validate(page) {
            if (!page.internal_id) {
                page.internal_id = uuid();
            }
            switch (true) {
                case page.primaryStyleId === null:
                    return false;
                case page.subStyleId === null:
                    return false;
                case page.selectedPerkIds.length < 9:
                    return false;
                case ![0, 1, 2, 3, 4, 5, 6, 7, 8].every((i) => page.selectedPerkIds[i] ?? false):
                    return false;
                case page.name.trim().length === 0:
                    return false;
                default:
                    return true;
            }
        },
        async loadPage(page) {
            let current_page = null;
            if (!this.validate(page)) {
                return;
            }
            await invoke("lcu", { endpoint: "/lol-perks/v1/currentpage", method: "GET" })
                .then((data) => (current_page = data))
                .catch(console.error);
            if (current_page.id) {
                await invoke("lcu", { endpoint: `/lol-perks/v1/pages/${current_page.id}`, method: "PUT", data: page }).catch(console.error);
            } else {
                await invoke("lcu", { endpoint: "/lol-perks/v1/pages", method: "POST", data: page }).catch(console.error);
            }

            this.$bus.emit("app.toast", { message: "Page loaded!" });
        },
        editPage(page) {
            console.log("EDIT", page);
            this.editing = page;
        },
        deletePage(page) {
            invoke("delete_page", { id: page.internal_id }).then((res) => {
                this.pages = res;
            });
        },
    },
};
</script>

<style scoped lang="less">
@import "@/assets/css/main.less";
// @import "@/assets/css/runes.less";

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
        // .other-rune,
        // .stat-mod {
        //     margin-right: 2px;
        // }
        .config {
            opacity: 0;
        }
        .options {
            opacity: 1;
        }
    }
}

.options {
    position: absolute;
    right: 4px;
    opacity: 0;
    transition: opacity 0.2s ease-in-out;
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
    transition: opacity 0.1s ease-in-out;
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
