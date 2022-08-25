<script setup lang="ts">
import settings from "iconoir/icons/settings.svg";
import upload from "iconoir/icons/upload.svg";
import download from "iconoir/icons/download.svg";
import trash from "iconoir/icons/trash.svg";
import prohibition from "iconoir/icons/prohibition.svg";
</script>


<template>
    <header data-tauri-drag-region class="flex-center connection">
        <div data-tauri-drag-region class="handle"></div>
        <div class="user flex-center">
            <template v-if="user.accountId">
                    <!-- <h3>Rift Buddy</h3> -->
                    <div class="pfp flex-center"><img :src="`https://ddragon.leagueoflegends.com/cdn/12.15.1/img/profileicon/${user.profileIconId}.png`" /></div>
                    <div class="username"><em class="sub-text">Logged in as {{user.displayName}}</em></div>
            </template>
            <template v-else>
                    <div class="pfp flex-center"><img src="https://ddragon.leagueoflegends.com/cdn/12.15.1/img/item/1001.png" /></div>
                    <div class="username"><em class="sub-text">LOL client not found...</em></div>
            </template>
        </div>
        <div class="ff"></div>
        <!-- <div class="tabs">
            <div class="tab" :class="{selected: tab === 'runes'}" @click="tab = 'runes'">Runes</div>
            <div class="tab" :class="{selected: tab === 'champ'}" @click="tab = 'champ'">Champ Selection</div>
        </div> -->
        <div class="ff"></div>
        <div class="nav-btns">
            <svg class="nav-btn" @click="() => minimize()" aria-hidden="false" width="12" height="12" viewBox="0 0 12 12"><rect fill="currentColor" width="10" height="1" x="1" y="6"></rect></svg>
            <svg class="nav-btn" @click="() => toggleMaximize()" aria-hidden="false" width="12" height="12" viewBox="0 0 12 12"><rect width="9" height="9" x="1.5" y="1.5" fill="none" stroke="currentColor"></rect></svg>
            <svg class="nav-btn" @click="() => close()" id="close" aria-hidden="false" width="12" height="12" viewBox="0 0 12 12"><polygon fill="currentColor" fill-rule="evenodd" points="11 1.576 6.583 6 11 10.424 10.424 11 6 6.583 1.576 11 1 10.424 5.417 6 1 1.576 1.576 1 6 5.417 10.424 1"></polygon></svg>
        </div>
    </header>
    <header style="height: 40px;">
        <div class="wordmark">
            <div class="logo"><img src="/logo.jpg" /></div>
        </div>
        <div class="search">
            <input type="text" :class="{'not-loading' : (results.length < 1 || !show_results) }"
            v-model="query" @keyup="find"
            @focus="show_results = true"
            @blur="() => close_results()"
            placeholder="Search for a champion" />
            <div class="results" v-if="results.length && show_results">
                <template v-if="loading">
                    <div class="loading"></div>
                </template>
                <div class="result" v-else-if="results.length" v-for="champ in results" @click="() => select(champ)" :key="champ.id">
                    <img :src="champ_img(champ.image.full)"/>
                    <div class="info">
                        <h4>{{champ.name}}</h4>
                        <p><em>{{champ.title}}</em></p>
                    </div>
                </div>
            </div>
        </div>
        <div class="options">
            <div class="button border icon" @click="() => clear()">
                Clear
                <img class="icon" :src="prohibition" />
            </div>
            <div class="button border icon" @click="saving_page = true">
                Save
                <img class="icon" :src="download" />
            </div>
            <div class="button border icon" @click="selecting_page = true">
                Load
                <img class="icon" :src="upload" />
            </div>
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
                <div class="spell" v-for="(spell, i) in champ.spells" :key="spell.id">
                    <h3>{{spells[i]}}</h3>
                    <img :title="`${spell.name}\n${spell.description}`" :src="spell_img(spell.image.full)" />
                </div>
            </div>
        </div>
        <div class="runes" v-if="runes.length">
            <div class="rune-page">
                <!-- PRIMARY STYLE -->
                <div class="select-main-rune">
                    <div class="main-rune" v-for="rune in runes" @click="() => select_main_rune(rune)"  :key="rune.id+'1'">
                        <img :title="rune.name" :src="rune_img(rune.icon)" :class="{'selected': page.primaryStyleId === rune.id}"/>
                    </div>
                </div>
                <!-- MAIN PAGE -->
                <template v-if="page.primaryStyleId">
                    <div class="rune-set" v-for="(slot, i) in primary_slots" :key="slot.runes.join(`1:${i}`)">
                        <div class="rune" :class="{'keystones': i === 0}" v-for="entry in slot.runes" @click="page.selectedPerkIds[i] = entry.id" :key="entry.id+'1'">
                            <img :title="entry.name" :class="{'selected': page.selectedPerkIds[i] === entry.id}" :src="rune_img(entry.icon)" />
                        </div>
                    </div>
                </template>
            </div>

            <div class="rune-page">
                <!-- SUB STYLE -->
                <div class="select-main-rune">
                    <template v-for="rune in runes">
                        <div class="main-rune" v-if="rune.id !== page.primaryStyleId" @click="page.subStyleId = rune.id" :key="rune.id+'2'">
                            <img :title="rune.name" :src="rune_img(rune.icon)" :class="{'selected': page.subStyleId === rune.id}"/>
                        </div>
                    </template>
                </div>
                <!-- SECONDARY PAGE -->
                <template v-if="page.subStyleId">
                    <div class="rune-set" v-for="(slot, i) in secondary_slots" :key="slot.runes.join(`2:${i}`)">
                    <template v-if="i !== 0">
                        <div class="rune" v-for="entry in slot.runes"  @click="() => select_secondary_perk(entry.id)" :key="entry.id+'2'">
                            <img :title="entry.name" :class="{'selected': page.selectedPerkIds.includes(entry.id)}" :src="rune_img(entry.icon)" />
                        </div>
                    </template>
                    </div>
                </template>
                <!-- STAT MODS -->
                <div class="flex-center" style="margin-top: 15px;">
                    <div class="stat-mod" v-for="mod in offense_stat_mods" @click="page.selectedPerkIds[6] = mod.id" :key="mod.id+'1'">
                        <img :title="mod.desc" :class="{'selected': page.selectedPerkIds[6] === mod.id}" :src="stat_img(mod.icon)" />
                    </div>
                </div>
                <div class="flex-center">
                    <div class="stat-mod" v-for="mod in flex_stat_mods" @click="page.selectedPerkIds[7] = mod.id"  :key="mod.id+'2'">
                        <img :title="mod.desc" :class="{'selected': page.selectedPerkIds[7] === mod.id}" :src="stat_img(mod.icon)" />
                    </div>
                </div>
                <div class="flex-center">
                    <div class="stat-mod" v-for="mod in defense_stat_mods" @click="page.selectedPerkIds[8] = mod.id" :key="mod.id+'3'">
                        <img :title="mod.desc" :class="{'selected': page.selectedPerkIds[8] === mod.id}" :src="stat_img(mod.icon)" />
                    </div>
                </div>
            </div>
        </div>
        <template v-if="loading">
            <div class="loading"></div>
        </template>
    </div>

    <Modal title="Load Page" @close="() => close_modal()" v-if="selecting_page">
        <div class="flex-center ff">
            <div class="search" style="margin-right: 8px;">
                <input type="text" :class="{'not-loading' : (page_results.length < 1 || !show_page_results) }"
                    @focus="() =>  find_page()"
                    @blur="() => close_results()"
                    v-model="page_query" @keyup="find_page"
                    placeholder="Search for a saved page" />
                <div class="results" v-if="page_results.length && show_page_results" style="max-height: 400px; overflow-y: auto;">
                    <div class="result" v-for="page in page_results" @click="() => select_page(page)" :key="page.internal_id">
                        <div class="info">
                            <h4>{{page.name}}</h4>
                            <!-- <p><em>{{page.metadata.champ.name}}</em></p> -->
                        </div>
                        <div class="ff"></div>
                        <div class="main-rune">
                            <img :src="rune_img(find_rune(page.primaryStyleId)?.icon)"/>
                        </div>
                        <div class="main-rune">
                            <img :src="rune_img(find_rune(page.subStyleId)?.icon)"/>
                        </div>
                    </div>
                </div>
            </div>
            <img class="icon" style="margin-right: 10px;" @click="() => delete_page()" :src="trash" />
        </div>
        <div class="options flex-center">
            <div class="button" @click="() => { if (selected_page) update(selected_page) }">Load</div>
            <div class="button" @click="() => { if (selected_page) page = {...selected_page}; selecting_page = false; }">Edit</div>
            <div class="button" @click="() => close_modal()">Close</div>
        </div>
    </Modal>
    <Modal title="Save Page" @close="saving_page = false" v-if="saving_page">
        <input v-model="page.name" placeholder="Enter a name for your page..." />
        <div class="options flex-center">
            <div class="button" @click="() => save()">Save</div>
            <div class="button" @click="saving_page = false">Close</div>
        </div>
    </Modal>
</template>

<script lang="ts">

import axios from "axios";
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { v4 as uuid } from 'uuid';
import { BaseDirectory, createDir, writeFile, readTextFile } from "@tauri-apps/api/fs";
import { defineComponent } from "vue";

const createDataFolder = async () => {
  return createDir("data", {
    dir: BaseDirectory.App,
    recursive: true,
  });
};

const createPagesFile = async () => {
  try {
    return writeFile(
      {
        contents: "[]",
        path: `./data/pages.json`,
      },
      {
        dir: BaseDirectory.App,
      }
    );
  } catch (e) {
    console.log(e);
  }
};

const readPagesFile = async (): Promise<Map<string, RunePage>> => {
    return readTextFile('./data/pages.json', { dir: BaseDirectory.App })
    .then(raw_pages => {
        try {
            return new Map(JSON.parse(raw_pages));
        } catch {
            return new Map();
        }
    })
    .catch(() => {
        return createPagesFile().then(() => new Map());
    });
}

const writeToFile = async (data: Map<string, any>) => {
    const raw = JSON.stringify(Array.from(data.entries()));
    try {
        return writeFile(
          {
            contents: raw,
            path: `./data/pages.json`,
          },
          {
            dir: BaseDirectory.App,
          }
        );
    } catch(e) {
        console.log(e)
    }
}

const writeItemToFile = async (item: RunePage) => {
    const new_pages = await readPagesFile();
    new_pages.set(item.internal_id, item);
    return writeToFile(new_pages);
}

type User = Record<string, any>;
type Champ = Record<string, any>;
type Rune = Record<string, any>;
type StatMod = Record<string, any>;
type RunePage = Record<string, any>;
type PageMetadata = Record<string, any>;

export default defineComponent({
    data() {
        return {
            spells: ["Q", "W", "E", "R"],
            offense_stat_mods: [
                {id: 5008, desc: "+9 Adaptive Force", icon: "StatModsAdaptiveForceIcon.png"},
                {id: 5005, desc: "+10 Attack Speed", icon: "StatModsAttackSpeedIcon.png"},
                {id: 5007, desc: "+8 Ability Haste", icon: "StatModsCDRScalingIcon.png"},
            ] as StatMod[],
            flex_stat_mods: [
                {id: 5008, desc: "+9 Adaptive Force", icon: "StatModsAdaptiveForceIcon.png"},
                {id: 5002, desc: "+6 Armor", icon: "StatModsArmorIcon.png"},
                {id: 5003, desc: "+8 Magic Resist", icon: "StatModsMagicResIcon.MagicResist_Fix.png"},
            ] as StatMod[],
            defense_stat_mods: [
                {id: 5001, desc: "+15-140 Health (based on level)", icon: "StatModsHealthScalingIcon.png"},
                {id: 5002, desc: "+6 Armor", icon: "StatModsArmorIcon.png"},
                {id: 5003, desc: "+8 Magic Resist", icon: "StatModsMagicResIcon.MagicResist_Fix.png"},
            ] as StatMod[],
            query: "",
            page_query: "",
            loading: false,
            updating: false,
            show_results: true,
            show_page_results: true,
            connected: false,
            interval: null as number | null,
            api_version: null,
            results: [] as Champ[],
            page_results: [] as RunePage[],
            champs: [] as Champ[],
            runes: [] as Rune[],
            champ: {} as Champ,
            user: {} as User,
            page: {
                name: "",
                primaryStyleId: null as number | null,
                subStyleId: null as number | null,
                selectedPerkIds: [] as number[],
                current: true,
                metadata: {} as PageMetadata,
                id: null as number | null,
                internal_id: null as number | null
            } as RunePage,
            current_page: {} as RunePage,
            selecting_page: false,
            selected_page: null as RunePage| null, 
            saving_page: false,
            saved_pages_map: new Map() as Map<string, RunePage>,
            secondary_perk_index: 4
        };
    }, 
    computed: {
        primary_slots(){
            // @ts-ignore This exists...
            return this.runes.find(rune => rune.id === this.page.primaryStyleId)?.slots ?? [];
        },
        secondary_slots(){
            // @ts-ignore This exists...
            return this.runes.find(rune => rune.id === this.page.subStyleId)?.slots ?? [];
        }
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
        close_results(){
            const _this = this;
            this.selected_page = null;
            setTimeout(() => _this.show_results = false, 300);
            setTimeout(() => _this.show_page_results = false, 300);
        },
        close_modal(){
            this.selecting_page = false;
            this.selected_page = null;
            this.page_query = '';
        },
        champ_img(champ_img_full: string) {
            return `https://ddragon.leagueoflegends.com/cdn/${this.api_version}/img/champion/${champ_img_full}`;
        },
        passive_img(passive_img_full: string) {
            return `https://ddragon.leagueoflegends.com/cdn/${this.api_version}/img/passive/${passive_img_full}`;
        },
        spell_img(spell_img_full: string) {
            return `https://ddragon.leagueoflegends.com/cdn/${this.api_version}/img/spell/${spell_img_full}`;
        },
        rune_img(rune_img_full: string) {
            return `./${rune_img_full}`;
        },
        stat_img(stat_img_full: string) {
            return `./perk-images/StatMods/${stat_img_full}`;
        },
        find(){
            if(this.query.trim().length < 1) { return this.results = [] };
            this.results = this.champs.filter((champ: Champ) => champ.name.toLowerCase().includes(this.query.toLowerCase())).slice(0, 10);
        },
        find_page(){
            this.show_page_results = true;
            this.selected_page = null;
            const pages = Array.from(this.saved_pages_map.values());
            if(this.page_query.trim().length < 1) { return this.page_results = pages };
            this.page_results = pages.filter((page: RunePage) => page.name.toLowerCase().includes(this.page_query.toLowerCase()));
        },
        select(champ: Champ){
            this.loading = true;
            this.query = '';
            this.results = [];
            axios.get(`https://ddragon.leagueoflegends.com/cdn/${this.api_version}/data/en_US/champion/${champ.id}.json`).then((res) => {
                this.champ = Object.values(res.data.data)[0] as Champ;
            }).finally(() => {
                this.loading = false;
            });
        },
        select_page(page: RunePage){
            this.page_query = page.name;
            this.page_results = [];
            this.selected_page = page;
        },
        select_main_rune(rune: any){
            if(this.page.subStyleId === rune.id){
                this.page.subStyleId = null;
            }
            this.page.primaryStyleId = rune.id;
        },
        select_secondary_perk(rune_id: number){
            if(!this.page.selectedPerkIds.includes(rune_id)){
                this.page.selectedPerkIds[this.secondary_perk_index] = rune_id;
                this.secondary_perk_index = this.secondary_perk_index === 4 ? 5 : 4;
            }
        },
        find_rune(rune_id: number){
            return this.runes.find(rune => rune.id === rune_id);
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
        clear(){
            this.page.name = "";
            this.page.primaryStyleId = null;
            this.page.subStyleId = null;
            this.page.selectedPerkIds = [];
            this.page.current = true;
            this.page.metadata = {};
            this.page.id = null;
            this.page.internal_id = null;
        },
        validate(page?: RunePage){
            const to_validate = page ?? this.page;
            if (!to_validate.internal_id) {
                to_validate.internal_id = uuid();
            }
            console.log({...to_validate});
            console.log(![0,1,2,3,4,5,6,7,8].every(i => to_validate.selectedPerkIds[i] ?? false));
            switch(true){
                case to_validate.primaryStyleId === null:
                    return false;
                case to_validate.subStyleId === null:
                    return false;
                case to_validate.selectedPerkIds.length < 9:
                    return false;
                case ![0,1,2,3,4,5,6,7,8].every(i => to_validate.selectedPerkIds[i] ?? false):
                    return false;
                case to_validate.name.trim().length === 0:
                    return false;
                default:
                    return true;
            }
        },
        async update(page?: RunePage){
            this.updating = true;
            const to_load = page ?? this.page;
            if(!this.validate(to_load)){return;}
            console.log(to_load);
            await invoke('lcu', {endpoint: '/lol-perks/v1/currentpage', method: 'GET'})
                .then((data) => this.current_page = data as RunePage).catch(console.error);
            if(this.current_page.id){
                await invoke('lcu', {endpoint: `/lol-perks/v1/pages/${this.current_page.id}`, method: 'PUT', data: to_load})
                    .catch(console.error);
            } else {
                await invoke('lcu', {endpoint: '/lol-perks/v1/pages', method: 'POST', data: to_load})
                    .then((data) => this.current_page = data as RunePage).catch(console.error);
            }
            this.updating = false;
            this.close_modal();
        },
        save(){
            if(!this.validate()){ return; }
            this.saved_pages_map.set(this.page.internal_id, {...this.page});
            writeToFile(this.saved_pages_map).finally(() => this.saving_page = false );
            this.clear();
        },
        delete_page(){
            if(!this.selected_page){ return; }
            this.saved_pages_map.delete(this.selected_page.internal_id);
            writeToFile(this.saved_pages_map);
            this.selected_page = null;
            this.page_query = '';
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
        createDataFolder().then(() => {
            readPagesFile().then(data => this.saved_pages_map = data);
        });
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
@import './assets/css/main.less';
@import './assets/css/runes.less';

.handle{
    position: absolute;
    top: 0;
    left: 0;
    width: 80%;
    height: 40px;
    z-index: 10;
}

.nav-btns{
    display: inline-flex;
    justify-content: center;
    align-items: center;
    height: 40px;
}

.nav-btn{
    height: 100%;
    padding: 0 15px;

    &:hover{
        cursor: pointer;
        background-color: fade(#222, 80%);
    }

    &#close:hover{
        background-color: fade(rgb(255, 0, 0), 70%);
    }
}

header {
    padding: 3px 15px;
}

header.connection {
    padding: 5px 15px;
    padding-right: 0;
    height: 30px;
    background-color: fade(#111, 50%);
    // background-color: fade(#333, 50%);
    // background: linear-gradient(90deg, fade(#333, 70%), fade(#111, 95%)), url('https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg') no-repeat center center;

    .tabs {
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 14px;
        line-height: 16px;
        font-weight: 600;
        .tab {
            white-space: nowrap;
            padding: 12px 10px;
            margin: 0 10px;
            position: relative;
            cursor: pointer;
            z-index: 20;
            // font-weight: bold;
            // border-bottom: @primary solid 2px;

            &:after {
                content: '';
                position: absolute;
                transform: scaleX(0);
                width: 100%;
                height: 2px;
                bottom: 0px;
                left: 0;
                background-color: @primary;
                transform-origin: middle;
                transition: transform 0.25s ease-out;
            }

            &.selected:after {
                transform: scaleX(1);
            }

            &:hover:after {
                transform: scaleX(1);
            }
        }
    }

    h3 {
        white-space: nowrap;
        margin-right: 10px;
        color: @primary;
    }
    .user {
        .pfp {
            img {
                border: fade(#111, 60%) solid 3px;
                background-color: fade(#111, 30%);
                border-radius: 999px;
                height: 25px;
            }
        }

        .username {
            white-space: nowrap;
            margin-left: 10px;
            font-size: 18px;
            .sub-text{
                font-size: 12px;
            }
        }
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

.main {
    margin: 10px 30px 0;
}

header {
    display: flex;
    justify-content: space-between;
    align-items: center;
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
        color: @primary;
    }
}

.logo {
    img {
        height: 30px;
    }
}
</style>
