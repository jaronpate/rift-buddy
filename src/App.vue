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
        <div class="champ" v-if="champ.id">
            <img class="profile" :src="champ_img(champ.image.full)" />
            <div class="info">
                <h2>{{champ.name}}</h2>
                <p><em>{{champ.tags.join(', ')}}</em></p>
            </div>
            <div class="ff"></div>
            <div class="info flex-left">
                <div class="spell">
                    <h3>P</h3>
                    <img :title="`${champ.passive.name}\n${champ.passive.description}`" :src="passive_img(champ.passive.image.full)" />
                </div>
                <div class="spell" v-for="(spell, i) in champ.spells">
                    <h3>{{spells[i]}}</h3>
                    <img :title="`${spell.name}\n${spell.description}`" :src="spell_img(spell.image.full)" />
                </div>
            </div>
        </div>
        <div class="runes" v-if="runes.length">
            <div class="rune-page">
                <div class="select-main-rune">
                    <div class="main-rune" v-for="rune in runes" @click="select_main_rune(rune)">
                        <img :title="rune.name" :src="rune_img(rune.icon)" :class="{'selected': page.main.path?.id === rune.id}"/>
                    </div>
                </div>
                <div class="rune-set" v-if="page.main.path" v-for="(slot, i) in page.main.path.slots">
                    <div class="rune" :class="{'keystones': i === 0}" v-for="entry in slot.runes">
                        <img :title="entry.name" :src="rune_img(entry.icon)" />
                    </div>
                </div>
            </div>

            
            <div class="rune-page">
                <div class="select-main-rune">
                    <template v-for="rune in runes">
                        <div class="main-rune" v-if="rune.id !== page.main.path?.id" @click="page.secondary.path = rune" >
                            <img :title="rune.name" :src="rune_img(rune.icon)" :class="{'selected': page.secondary.path?.id === rune.id}"/>
                        </div>
                    </template>
                </div>
                <div class="rune-set" v-if="page.secondary.path" v-for="(slot, i) in page.secondary.path.slots">
                    <div class="rune" v-if="i !== 0" v-for="entry in slot.runes">
                        <img :title="entry.name" :src="rune_img(entry.icon)" />
                    </div>
                </div>
            </div>
        </div>
        <template v-if="loading">
            <div class="loading"></div>
        </template>
    </div>
</template>

<script lang="ts">
import axios from "axios";
import { invoke } from '@tauri-apps/api/tauri';

type Champ = Record<string, any>;
type Rune = Record<string, any>;
type ConnData = Record<string, any>;

export default {
    data() {
        return {
            spells: ["Q", "W", "E", "R"],
            query: "",
            loading: false,
            api_version: null,
            conn_data: {} as ConnData,
            results: [] as Champ[],
            champs: [] as Champ[],
            runes: [] as Rune[],
            champ: {} as Champ,
            page: {
                name: "Test",
                main: {
                    path: {} as Rune | null,
                    keystone: {} as Rune | null,
                },
                secondary: {
                    path: {} as Rune | null,
                    keystone: {} as Rune | null,
                }
            }
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
        rune_img(rune_img_full: string) {
            return `https://ddragon.canisback.com/img/${rune_img_full}`;
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
                this.champ = Object.values(res.data.data)[0] as Champ;
                console.log(this.champ)
            }).finally(() => {
                this.loading = false;
            });
        },
        select_main_rune(rune: any){
            if(this.page.secondary.path?.id === rune.id){
                this.page.secondary.path = null;
            }
            this.page.main.path = rune;
        }
    },
    mounted() {
        invoke('get_credentials')
            .then((data) => {
                this.conn_data = data as ConnData;
                console.log(this.conn_data)
            }).catch(console.error);
        invoke('lcu', {endpoint: '/lol-summoner/v1/current-summoner'})
            .then((data) => {
                console.log(data)
            }).catch(console.error);
        axios.get("https://ddragon.leagueoflegends.com/api/versions.json").then((res) => {
            this.api_version = res.data[0];
        }).then(() => {
            axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.api_version}/data/en_US/champion.json`).then((res) => {
                this.champs = Object.values(res.data.data);
            });
            axios.get(`http://ddragon.leagueoflegends.com/cdn/${this.api_version}/data/en_US/runesReforged.json`).then((res) => {
                this.runes = res.data;
                console.log(this.runes)
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

.runes {
    margin-top: 25px;
    display: flex;
    justify-content: space-around;
    align-items: flex-start;
}

.rune-page {
    width: 35%;
}

.rune-set {
    display: flex;
    justify-content: space-around;
    align-items: center;
}

.select-main-rune{
    margin: 15px 0;
    .main-rune{
        display: inline;
        margin: 0 5px;

        img {
            border: #111 solid 2px;
            background-color: fade(#111, 50%);
            padding: 6px;
            width: 18px;
            border-radius: 99px;
            transition: outline linear 0.1s;
            outline-color: #b39556;
            outline: #d6414100 solid 2px;

            &:hover, &.selected {
                outline-color: #b39556;
                opacity: 0.9;
                cursor: pointer;
            }
        }
    }
}

.rune {
    display: inline;
    margin: 0 5px;

    &.keystones {
        margin: 0 5px 15px 5px;
    }

    img {
        border: #111 solid 2px;
        background-color: fade(#111, 50%);
        padding: 1px;
        border-radius: 99px;
        width: 45px;
        transition: outline linear 0.1s;
        outline-color: #b39556;
        outline: #d6414100 solid 2px;

        &:hover{
            outline-color: #b39556;
            opacity: 0.9;
            cursor: pointer;
        }
    }
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
            font-size: 28px;
        }
        em {
            font-size: 0.8em;
            color: #999;
        }
        p {
            margin: 0;
        }
        .spell {
            margin-top: -15px;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;

            h3 {
                margin: 0;
                font-size: 0.9em;
                // color: #999;
            }

            img {
                width: 40px;
                margin: 0 5px;
                border-radius: 5px;
                border: #111 solid 2px;
            }
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
        z-index: 10;

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

.options {
    display: flex;
    justify-content: center;
    align-items: center;
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
