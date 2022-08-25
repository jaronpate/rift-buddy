<script setup lang="ts">
</script>


<template>
    <div class="overlay">
        <div class="logo"><img src="./assets/logo.jpg" /></div>
    </div>
</template>

<script lang="ts">

import axios from "axios";
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { v4 as uuid } from 'uuid';
import { defineComponent } from "vue";

type User = Record<string, any>;
type Champ = Record<string, any>;
type Rune = Record<string, any>;
type StatMod = Record<string, any>;
type RunePage = Record<string, any>;
type PageMetadata = Record<string, any>;

export default defineComponent({
    data() {
        return {
            api_version: null,
            interval: null as number | null,
            user: {} as User,
            connected: false,
            current_page: {} as RunePage,
            champs: [] as Champ[],
            runes: [] as Rune[]
        };
    }, 
    computed: {
        
    },
    methods: {
        minimize(){
            appWindow.minimize();
        },
        toggleMaximize(){
            appWindow.toggleMaximize();
        },
        close(){
            appWindow.close();
        },
        setup(){
            invoke('lcu', {endpoint: '/lol-summoner/v1/current-summoner', method: 'GET'})
                .then((data) => {
                    this.user = data as User;
                }).catch(console.error);
            invoke('lcu', {endpoint: '/lol-perks/v1/currentpage', method: 'GET'})
                .then((data) => {
                    this.current_page = data as RunePage;
                }).catch(console.error);
        },
        connect(){
            invoke('get_credentials')
                .then(() => {
                    if(!this.user.accountId) { this.setup(); }
                    this.connected = true;
                }).catch(() => {
                    this.user = {};
                    this.connected = false;
                });
        }
    },
    mounted() {
        this.connect();
        // @ts-ignore idk what ts is thinking here
        this.interval = setInterval(this.connect, 2000);
        axios.get("https://ddragon.leagueoflegends.com/api/versions.json").then((res) => {
            this.api_version = res.data[0];
        }).then(() => {
            axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.api_version}/data/en_US/champion.json`).then((res) => {
                this.champs = Object.values(res.data.data);
            });
            axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.api_version}/data/en_US/runesReforged.json`).then((res) => {
                this.runes = res.data;
            });
        })        
    },
    unmounted(){
        clearInterval(this.interval ?? undefined);
    }
});
</script>

<style scoped lang="less">
@primary: #b39556;

@font-face {
    font-family: "Inter", sans-serif;
    src: url("./assets/Inter.ttf") format("ttf");
}

.overlay {
    height: 100%;
    width: 100%;
    padding: 10px;
    border-radius: 10px;
    background-color: fade(#333, 70%);
}

.logo {
    img {
        width: 32px;
    }
}
</style>
