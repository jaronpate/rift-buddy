<template>
    <div class="tab-title">
        <div class="flex flex-align flex-row">
            <div>
                <h2>Create New</h2>
                <p>Create and save a page right here in Rift Buddy</p>
            </div>
            <div class="ff"></div>
            <div class="button" @click="savePage()">Save</div>
        </div>
    </div>
    <div class="metadata flex flex-align">
        <div class="meta ff">
            <label>Name</label>
            <input autocomplete="new-password" v-model="page.name" placeholder="Page Nickname" />
        </div>
        <div class="meta ff">
            <label>Associated Champion</label>
            <ChampSearch :value="page.associatedChampion" @select="selectChamp" />
        </div>
    </div>
    <div class="create">
        <div class="runes" v-if="runes.length">
            <div class="rune-page">
                <!-- PRIMARY STYLE -->
                <div class="select-main-rune">
                    <div class="main-rune" v-for="rune in runes" @click="() => selectMainRune(rune)" :key="rune.id + '1'">
                        <img :title="rune.name" :src="runeImg(rune)" :class="{ selected: page.primaryStyleId === rune.id }" />
                    </div>
                </div>
                <!-- MAIN PAGE -->
                <template v-if="page.primaryStyleId">
                    <div class="rune-set" v-for="(slot, i) in primary_slots" :key="slot.runes.join(`1:${i}`)">
                        <div
                            class="rune"
                            :class="{ keystones: i === 0 }"
                            v-for="entry in slot.runes"
                            @click="page.selectedPerkIds[i] = entry.id"
                            :key="entry.id + '1'"
                        >
                            <img :title="entry.name" :class="{ selected: page.selectedPerkIds[i] === entry.id }" :src="runeImg(entry)" />
                        </div>
                    </div>
                </template>
            </div>

            <div class="rune-page">
                <!-- SUB STYLE -->
                <div class="select-main-rune">
                    <template v-for="rune in runes">
                        <div
                            class="main-rune"
                            v-if="rune.id !== page.primaryStyleId"
                            @click="page.subStyleId = rune.id"
                            :key="rune.id + '2'"
                        >
                            <img :title="rune.name" :src="runeImg(rune)" :class="{ selected: page.subStyleId === rune.id }" />
                        </div>
                    </template>
                </div>
                <!-- SECONDARY PAGE -->
                <template v-if="page.subStyleId">
                    <div class="rune-set" v-for="(slot, i) in secondary_slots" :key="slot.runes.join(`2:${i}`)">
                        <template v-if="i !== 0">
                            <div
                                class="rune"
                                v-for="entry in slot.runes"
                                @click="() => selectSecondaryPerk(entry.id)"
                                :key="entry.id + '2'"
                            >
                                <img
                                    :title="entry.name"
                                    :class="{ selected: page.selectedPerkIds.includes(entry.id) }"
                                    :src="runeImg(entry)"
                                />
                            </div>
                        </template>
                    </div>
                </template>
                <!-- STAT MODS -->
                <div class="flex-center" style="margin-top: 15px">
                    <div class="stat-mod" v-for="mod in offense_stat_mods" @click="page.selectedPerkIds[6] = mod.id" :key="mod.id + '1'">
                        <img :title="mod.desc" :class="{ selected: page.selectedPerkIds[6] === mod.id }" :src="runeImg(mod)" />
                    </div>
                </div>
                <div class="flex-center">
                    <div class="stat-mod" v-for="mod in flex_stat_mods" @click="page.selectedPerkIds[7] = mod.id" :key="mod.id + '2'">
                        <img :title="mod.desc" :class="{ selected: page.selectedPerkIds[7] === mod.id }" :src="runeImg(mod)" />
                    </div>
                </div>
                <div class="flex-center">
                    <div class="stat-mod" v-for="mod in defense_stat_mods" @click="page.selectedPerkIds[8] = mod.id" :key="mod.id + '3'">
                        <img :title="mod.desc" :class="{ selected: page.selectedPerkIds[8] === mod.id }" :src="runeImg(mod)" />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import axios from "axios";
import { v4 as uuid } from "uuid";
import { invoke } from "@tauri-apps/api/tauri";
import ChampSearch from "@/components/ChampSearch.vue";

export default {
    name: "CreatePage",
    components: {
        ChampSearch,
    },
    data: () => {
        return {
            page: {
                name: "",
                associatedChampion: null,
                primaryStyleId: null,
                subStyleId: null,
                selectedPerkIds: [],
                current: true,
                metadata: {},
                id: null,
                internal_id: null,
            },
            runes: [],
            offense_stat_mods: [
                { id: 5008, desc: "+9 Adaptive Force", icon: "perk-images/StatMods/StatModsAdaptiveForceIcon.png" },
                { id: 5005, desc: "+10 Attack Speed", icon: "perk-images/StatMods/StatModsAttackSpeedIcon.png" },
                { id: 5007, desc: "+8 Ability Haste", icon: "perk-images/StatMods/StatModsCDRScalingIcon.png" },
            ],
            flex_stat_mods: [
                { id: 5008, desc: "+9 Adaptive Force", icon: "perk-images/StatMods/StatModsAdaptiveForceIcon.png" },
                { id: 5002, desc: "+6 Armor", icon: "perk-images/StatMods/StatModsArmorIcon.png" },
                { id: 5003, desc: "+8 Magic Resist", icon: "perk-images/StatMods/StatModsMagicResIcon.MagicResist_Fix.png" },
            ],
            defense_stat_mods: [
                { id: 5001, desc: "+15-140 Health (based on level)", icon: "perk-images/StatMods/StatModsHealthScalingIcon.png" },
                { id: 5002, desc: "+6 Armor", icon: "perk-images/StatMods/StatModsArmorIcon.png" },
                { id: 5003, desc: "+8 Magic Resist", icon: "perk-images/StatMods/StatModsMagicResIcon.MagicResist_Fix.png" },
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
        primary_slots() {
            return this.runes.find((rune) => rune.id === this.page.primaryStyleId)?.slots ?? [];
        },
        secondary_slots() {
            return this.runes.find((rune) => rune.id === this.page.subStyleId)?.slots ?? [];
        },
    },
    async mounted() {
        await axios.get("https://ddragon.leagueoflegends.com/api/versions.json").then((res) => {
            this.ddversion = res.data[0];
        });

        await axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.ddversion}/data/en_US/runesReforged.json`).then((res) => {
            this.runes = res.data;
        });
    },
    methods: {
        findRune(rune_id) {
            console.log(rune_id);
            return this.flattened.find((rune) => rune.id === rune_id);
        },
        runeImg(rune) {
            return `./${rune.icon.replace("perk-images", "images/perks")}`;
        },
        selectMainRune(rune) {
            if (this.page.subStyleId === rune.id) {
                this.page.subStyleId = null;
            }
            this.page.primaryStyleId = rune.id;
        },
        selectSecondaryPerk(rune_id) {
            if (!this.page.selectedPerkIds.includes(rune_id)) {
                this.page.selectedPerkIds[this.secondary_perk_index] = rune_id;
                this.secondary_perk_index = this.secondary_perk_index === 4 ? 5 : 4;
            }
        },
        selectChamp(champion) {
            this.page.associatedChampion = champion ? champion.id : null;
        },
        clear() {
            this.page = {
                name: "",
                primaryStyleId: null,
                subStyleId: null,
                selectedPerkIds: [],
                current: true,
                metadata: {},
                id: null,
                internal_id: null,
            };
        },
        savePage() {
            this.page.internal_id = uuid();
            invoke("save_page", { id: this.page.internal_id, page: this.page }).then((res) => {
                this.clear();
                this.$bus.emit("app.toast", { message: "Page saved!" });
            });
        },
    },
};
</script>

<style scoped lang="less">
@import "@/assets/css/main.less";
@import "@/assets/css/runes.less";

.metadata {
    margin: 0 -5px 10px -5px;

    .meta {
        margin: 0 5px;

        label {
            font-size: 13px;
            line-height: 16px;
            font-weight: 500;
            // color: #aaa;
        }
    }
}

.select-main-rune {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-evenly;
}
</style>
