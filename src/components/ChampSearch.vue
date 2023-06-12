<template>
    <div class="search" :class="{ active: show_results }" ref="$parent">
        <template v-if="champ && !show_results">
            <div class="champ flex flex-align" @click.stop="edit">
                <img :src="champImg(champ.image.full)" />
                <div class="name">
                    {{ champ.name }}
                </div>
                <div class="ff"></div>
                <div class="clear" @click.stop="$emit('select', null)">
                    <Cancel width="16px" height="16px" color="white" />
                </div>
            </div>
        </template>
        <template v-else>
            <input
                type="text"
                :class="{ 'not-loading': results.length < 1 || !show_results }"
                v-model="query"
                @keyup="find"
                @focus="show_results = true"
                @blur="() => closeResults()"
                placeholder="Search for a champion"
                ref="search"
                autocomplete="new-password"
            />
            <div class="results" v-if="results.length && show_results">
                <template v-if="loading">
                    <div class="loading"></div>
                </template>
                <div
                    class="result"
                    v-else-if="results.length"
                    v-for="champ in results"
                    @mousedown="() => selectChamp(champ)"
                    :key="champ.id"
                >
                    <img :src="champImg(champ.image.full)" />
                    <div class="info">
                        <h4>{{ champ.name }}</h4>
                        <p>
                            <em>{{ champ.title }}</em>
                        </p>
                    </div>
                </div>
            </div>
        </template>
    </div>
</template>

<script>
import axios from "axios";
import { Cancel } from "@iconoir/vue";

export default {
    name: "ChampSearch",
    components: {
        Cancel,
    },
    data: () => {
        return {
            query: "",
            show_results: false,
            loading: false,
            results: [],
            champs: [],
            ddversion: null,
        };
    },
    computed: {
        champ() {
            return this.champs.find((champ) => champ.id === this.value);
        },
    },
    props: {
        value: {
            type: String,
            default: null,
        },
    },
    async mounted() {
        window.addEventListener("keydown", (e) => {
            if (e.key === "Escape") {
                this.show_results = false;
            }
        });
        window.addEventListener("click", (e) => {
            if (this.$refs.$parent && !this.$refs.$parent.contains(e.target)) {
                this.show_results = false;
            }
        });
        // Get api version
        await axios.get("https://ddragon.leagueoflegends.com/api/versions.json").then((res) => {
            this.ddversion = res.data[0];
        });
        // Get all champions
        await axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.ddversion}/data/en_US/champion.json`).then((res) => {
            this.champs = Object.values(res.data.data);
        });
    },
    methods: {
        selectChamp(champ) {
            this.query = "";
            this.show_results = false;
            this.$emit("select", champ);
        },
        champImg(champ_img_full) {
            return `https://ddragon.leagueoflegends.com/cdn/${this.ddversion}/img/champion/${champ_img_full}`;
        },
        closeResults() {
            this.show_results = false;
        },
        edit() {
            this.show_results = true;
            this.$nextTick(() => {
                this.$refs.search.focus();
            });
        },
        find() {
            if (this.query.trim().length < 1) {
                return (this.results = []);
            }
            this.results = this.champs.filter((champ) => champ.name.toLowerCase().includes(this.query.toLowerCase())).slice(0, 10);
        },
    },
};
</script>

<style scoped lang="less">
@import "@/assets/css/main.less";

.champ {
    background-color: #0f0f0f98;
    border-radius: 5px;
    height: 33px;
    padding: 0 5px;
    cursor: pointer;

    .name {
        font-size: 14px;
        font-weight: 500;
        color: #fff;
        line-height: 33px;
    }

    .clear {
        margin-left: auto;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 18px;
        height: 18px;
        min-width: 18px;
        min-height: 18px;
        border-radius: 50%;
        background-color: #333;
        cursor: pointer;
    }

    img {
        width: 23px;
        height: 23px;
        margin-right: 8px;
    }
}
</style>
