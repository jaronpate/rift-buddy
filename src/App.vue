<script setup lang="ts">
import settings from "iconoir/icons/settings.svg";
import info from "iconoir/icons/info-empty.svg";
import search from "iconoir/icons/search.svg";
</script>

<template>
    <header>
        <div class="wordmark">
            <div class="logo"><img src="./assets/logo.jpg" /></div>
            <!-- <h3>Rift Buddy</h3> -->
        </div>
        <div class="search">
            <input type="text" :class="{'not-loading' : (!loading && results.length < 1) }" v-model="query" @keyup="find" placeholder="Search for a champion" />
            <div class="results" v-if="results.length && !loading">
                <template v-if="loading">
                    <div class="loading"></div>
                </template>
                <div class="result" v-else-if="results.length" v-for="champ in results" @click="select(champ)">
                    <img :src="champ_img(champ.image.full)"/>
                    <div class="info">
                        <h4>{{champ.name}}</h4>
                        <p><em>{{champ.title}}</em></p>
                    </div>
                </div>
            </div>
        </div>
        <div class="options">
            <img class="icon" :src="settings" />
        </div>
    </header>
    <div class="main">

        <div class="champ" v-if="champ">
            <img class="profile" :src="champ_img(champ.image.full)" />
            <div class="info">
                <h2>{{champ.name}}</h2>
                <p><em>{{champ.tags.join(', ')}}</em></p>
            </div>
            <div class="ff"></div>
            <div class="info flex-left">
                <img class="spell" :title="`${champ.passive.name}\n${champ.passive.description}`" :src="passive_img(champ.passive.image.full)" />
                <img class="spell" :title="`${spell.name}\n${spell.description}`" v-for="spell in champ.spells" :src="spell_img(spell.image.full)" />
            </div>
        </div>
        <template v-else-if="loading">
            <div class="loading"></div>
        </template>
    </div>
</template>

<script lang="ts">
import axios from "axios";

type Champ = Record<string, any>;

export default {
    data() {
        return {
            query: "",
            loading: false,
            api_version: null,
            results: [],
            champs: [],
            champ: null
        };
    },
    methods: {
        champ_img(champ_img_full: string) {
            return `http://ddragon.leagueoflegends.com/cdn/${this.api_version}/img/champion/${champ_img_full}`;
        },
        passive_img(passive_img_full: string) {
            return `http://ddragon.leagueoflegends.com/cdn/${this.api_version}/img/passive/${passive_img_full}`;
        },
        spell_img(spell_img_full: string) {
            return `http://ddragon.leagueoflegends.com/cdn/${this.api_version}/img/spell/${spell_img_full}`;
        },
        find(){
            if(this.query.trim().length < 1) { return this.results = [] };
            this.results = this.champs.filter((champ: Champ) => champ.name.toLowerCase().includes(this.query.toLowerCase())).slice(0, 10);
        },
        select(champ: Champ){
            this.loading = true;
            this.query = '';
            this.results = [];
            axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.api_version}/data/en_US/champion/${champ.id}.json`).then((res) => {
                this.champ = Object.values(res.data.data)[0];
                console.log(this.champ)
            }).finally(() => {
                this.loading = false;
            });
        }
    },
    mounted() {
        axios.get("https://ddragon.leagueoflegends.com/api/versions.json").then((res) => {
            this.api_version = res.data[0];
        }).then(() => {
            axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.api_version}/data/en_US/champion.json`).then((res) => {
                this.champs = Object.values(res.data.data);
            });
        })        
    },
};
</script>

<style scoped lang="less">
@font-face {
    font-family: "Inter", sans-serif;
    src: url("./assets/Inter.ttf") format("ttf");
}

.flex-center {
    display: flex;
    justify-content: center;
    align-items: center;
}

.flex-left {
    display: flex;
    justify-content: flex-start;
    align-items: center;
}

.ff {
    width: 100%;
}

.loading {
    border: 5px solid fade(#222, 30%); /* Light grey */
    border-top: 5px solid fade(#222, 60%);
    border-radius: 50%;
    width: 20px;
    height: 20px;
    animation: spin 2s linear infinite;
    margin: auto;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}

input {
    font-family: "Inter", sans-serif;
    -moz-box-sizing: border-box;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
    border: 0;
    outline: 0;
    padding: 10px 10px;
    border-radius: 5px;
    width: 100%;

    &.not-loading {
        border-bottom-left-radius: 5px !important;
        border-bottom-right-radius: 5px !important;
    }
}

.champ {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    height: 100%;
    width: 100%;
    .profile {
        width: 10%;
        box-shadow: #111 0px 0px 10px;
    }
    .info {
        text-align: left;
        margin-left: 20px;
        h2 {
            margin: 0;
        }
        em {
            font-size: 0.8em;
            color: #999;
        }
        p {
            margin: 0;
        }
        .spell {
            width: 40px;
            margin: 0 5px;
            border-radius: 5px;
            border: #111 solid 2px;
        }
    }
}

.search {
    width: 100%;
    margin: auto 15px;
    position: relative;

    .results {
        -moz-box-sizing: border-box;
        -webkit-box-sizing: border-box;
        box-sizing: border-box;
        background-color: #333;
        border-radius: 5px;
        border-top-left-radius: 0;
        border-top-right-radius: 0;
        width: 100%;
        padding: 5px 0;
        position: absolute;
        text-align: left;

        .result {
            padding: 5px 10px;
            display: flex;
            justify-content: flex-start;
            align-items: center;
            .info {
                margin-left: 15px;
            }
            h4,
            p {
                line-height: 18px;
                margin: 0;
            }
            h4 {
                font-weight: 600;
            }
            em {
                font-size: 0.8em;
                color: #999;
            }
            img {
                width: 40px;
            }

            &:last-of-type {
                margin-bottom: 0;
            }
            &:hover {
                background-color: fade(#222, 30%);
                cursor: pointer;
            }
        }
    }

    input {
        border-bottom-left-radius: 0;
        border-bottom-right-radius: 0;
    }
}

.main {
    margin: 10px 30px 0;
}

.icon {
    filter: invert(0.9);
    margin-left: 6px;
    transition: filter 0.1s linear;

    &:hover {
        filter: invert(0.7);
        cursor: pointer;
    }
}

header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    // background-color: fade(#333, 50%);
    // background: linear-gradient(90deg, fade(#333, 70%), fade(#111, 95%)), url('http://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg') no-repeat center center;
    padding: 10px 15px;
}

.wordmark {
    display: flex;
    align-items: center;
    justify-content: center;

    h3 {
        margin: 0 0 0 10px;
        color: #b39556;
    }
}

.logo {
    img {
        height: 30px;
    }
}
</style>
