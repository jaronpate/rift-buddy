<template>
  <div>
    
    <div id="app" style="margin-top: 15px">

      <div class="profile">
        <span class="profileIcon">
          <img v-bind:src="summoner.icon" style="max-height: 64px; border-radius: 50%;">
          <span id="level">{{ summoner.summonerLevel }}</span>
        </span>
        <h1>{{ summoner.displayName }}</h1>
      </div>

      <!-- <img v-bind:src="champion.icon" style="max-height: 64px;">
      <h1>{{ champion.name }}</h1> -->
      
      <div class="perk-select" style="margin-bottom: 15px;">
        Rune Page: <select v-model="selectedPageName" id="perkpage">
          <option selected disabled hidden>Select a rune page...</option>
          <option v-for="page in perkPages" v-bind:key="page.id">{{ page.name }}</option>
        </select>
        <button @click="savePage">Save Page</button>

        <div class="styles">
          <p>Primary Rune: {{ selectedPrimary.name }}</p>
          <ul>
            <li v-for="perk in primaryPerks" v-bind:key="perk.id">
              {{ perk.name }}
            </li>
          </ul>
          <p>Secondary Rune: {{ selectedSecondary.name }}</p>
          <ul>
            <li v-for="perk in secondaryPerks" v-bind:key="perk.id">
              {{ perk.name }}
            </li>
          </ul>
        </div>
      </div>

      <hr>

      <div class="stored-perk-select">
        Stored Pages: <select v-model="selectedStoredPageName" id="storedperkpage">
          <option selected disabled hidden>Select a rune page...</option>
          <option v-for="page in storedPages" v-bind:key="page.name">{{ page.name }}</option>
        </select>
        <button @click="loadPage">Load Page</button>
        <button @click="removePage">Delete Page</button>
      </div>

    </div>

    <footer>
      <span style="margin-right: 10px">LCU - {{version}} | {{appName}} - {{appVer}}</span>
    </footer>
  </div>
</template>

<script>
import riot from './lib/riotApi.js'
const invoke = window.__TAURI__.invoke;
const app = window.__TAURI__.app;

let ddVer;

// Window titlebar functionality
import { appWindow } from '@tauri-apps/api/window'

document
  .getElementById('minimize')
  .addEventListener('click', () => appWindow.minimize())
document
  .getElementById('maximize')
  .addEventListener('click', () => appWindow.toggleMaximize())
document
  .getElementById('close')
  .addEventListener('click', () => appWindow.close())

export default {
  name: 'App',
  data: function() {
    return {
      selectedPageName: "Select a rune page...",
      selectedStoredPageName: "Select a rune page...",
      storedPages: [],
      perkPages: {},
      selectedPage: {},
      selectedPerks: [],
      primaryPerks: [],
      secondaryPerks: [],
      selectedPrimary: {},
      selectedSecondary: {},
      perks: {},
      styles: {},
      connData: {}
    }
  },
  created(){
    let storedPages = window.localStorage.getItem("pages")
    if (storedPages) {
      this.storedPages = JSON.parse(storedPages);
    }
  },
  asyncComputed: {
    async summoner() {
      let connData = await invoke('get_credentials')
        .then(data => data).catch(e => {
          console.log(e)
          invoke("error");
        });
      console.log(connData);
      this.connData = connData
      let sum =  await invoke('lcu', {
        endpoint: '/lol-summoner/v1/current-summoner',
        port: connData.port,
        password: connData.password
      }).then((data) => data).catch(e => console.log(e));

      // Get other LCU data
      let perks = await invoke('lcu', {
        endpoint: `/lol-perks/v1/perks`,
        port: connData.port,
        password: connData.password
      }).then(data => data);

      let styles = await invoke('lcu', {
        endpoint: `/lol-perks/v1/styles`,
        port: connData.port,
        password: connData.password
      }).then(data => data);

      let sumPerks = await invoke('lcu', {
        endpoint: `/lol-perks/v1/pages`,
        port: connData.port,
        password: connData.password
      }).then(data => data);
      sumPerks = sumPerks.filter(page => page.isDeletable === true);

      this.perks = perks;
      this.styles = styles;
      this.perkPages = sumPerks;
      console.log(sumPerks)

      sum.icon = `http://ddragon.leagueoflegends.com/cdn/${ddVer}/img/profileicon/${sum.profileIconId}.png`
      return sum;
    },
    async version() {
      return await riot("get", "/realms/na.json")
        .then(data => data.v)
    },
    async appName() {
      return await app.getName()
        .then(data => data)
    },
    async appVer() {
      return await app.getVersion()
        .then(data => data)
    },
    champions: {
      async get() {
        return await riot("get", `/cdn/${ddVer}/data/en_US/champion.json`)
          .then((res) => res.data)
      },
      lazy: true
    }
  },
  methods:{
    savePage(){
      window.localStorage.setItem(this.selectedPage.name, this.selectedPage)
      let pages = JSON.parse(window.localStorage.getItem("pages"))
      if (pages !== null) {
        pages.push(this.selectedPage)
        window.localStorage.setItem("pages", JSON.stringify(pages))
      } else {
        window.localStorage.setItem("pages", JSON.stringify([this.selectedPage]))
      }
      this.storedPages = pages;
    },
    getPages(){
      let pages = JSON.parse(window.localStorage.getItem("pages"))
      return pages
    },
    loadPage(){
      const self = this;
      let data = this.storedPages.find(test => test.name === this.selectedStoredPageName);
      invoke('post', {
        endpoint: `/lol-perks/v1/pages/${data.id}`,
        port: self.connData.port,
        password: self.connData.password,
        data: data
      }).then((data) => data).catch(e => console.log(e));
    },
    removePage(){
      let pages = JSON.parse(window.localStorage.getItem("pages"));
      let i = pages.findIndex(test => test.name === this.selectedStoredPageName);
      console.log(i)
      console.log(this.selectedStoredPageName)
      console.log(pages)
      if(i !== -1){
        pages.splice(i, 1);
        window.localStorage.setItem("pages", JSON.stringify(pages));
        this.storedPages = pages;
      }
    }
  },
  watch: {
    summoner(data) {
      console.log(data)
    },
    selectedPageName(data) {
      // Setup Perk Picker upon page select
      const self = this;
      this.selectedPage = this.perkPages.find(page => page.name === data)
      console.log(this.selectedPage)
      this.selectedPerks = [];
      this.selectedPage.selectedPerkIds.forEach(function(perk){
        let foundPerk = self.perks.find(test => test.id === perk)
        foundPerk.icon = `https://${self.connData.address}:${self.connData.port}${foundPerk.iconPath}`
        self.selectedPerks.push(foundPerk)
      })
      this.primaryPerks = this.selectedPerks.slice(0,4)
      this.secondaryPerks = this.selectedPerks.slice(4,9)
      // console.log(this.primaryPerks, this.secondaryPerks)
      this.selectedPrimary = this.styles.find(test => test.id === this.selectedPage.primaryStyleId)
      this.selectedSecondary = this.styles.find(test => test.id === this.selectedPage.subStyleId)
      console.log(this.perkPages)
    },
    version(vv){
      console.log(vv)
      ddVer = vv
      this.$asyncComputed.champions.update();
      riot("get", `/cdn/${vv}/data/en_US/champion.json`).then(function (res) {
        let champs = Object.keys(res.data);
        let champ = champs[Math.floor(Math.random() * ((champs.length - 1) - 0) + 0)]
        document.body.style.background = `linear-gradient(0deg, rgba(45, 45, 45, 0.94), rgba(0, 0, 0, 0.94)), 
        url(${`http://ddragon.leagueoflegends.com/cdn/img/champion/splash/${res.data[champ].id}_0.jpg`})`
      });
    }
  }
}
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=Source+Sans+Pro:wght@300;400;600&display=swap');
html{
  background-size:cover;
  height: 100%;
  overflow: hidden;
}
body{
  font-family: 'Source Sans Pro', sans-serif;
  margin: auto;
  padding: 2rem;
  color: white;
  background-position: center !important;
  background-size: cover !important;
}

.profile{
  display: flex;
  flex-direction: row;
  justify-content: left;
  align-items: center;
  margin-bottom: 15px;
}

.profile img{
  border: 3px solid rgba(128, 128, 128, 0.35);
  padding: 3px;
}

.profile h1{
  margin-left: -10px;
}

#level{
  font-size: 1em;
  margin: 0;
  position: relative;
  background: #141414;
  padding: 3px 6px;
  border-radius: 100px;
  position: relative;
  left: -50%;
}

nav{
  background-color: #323233;
  height: 30px;
  width: 100%;
  position: absolute;
  top: 0;
  left: 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.nav-logo{
  margin-left: 10px;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}

.nav-btns{
  -webkit-app-region: no-drag;
}

.nav-btns i, .nav-btns img, .nav-btns svg{
  padding: 9px 3px;
  width: 26px;
  height: 0.9em;
  text-align: center;
}

.nav-btns i:hover, .nav-btns img:hover, .nav-btns svg:hover{
  background-color: #414141;
  cursor: pointer;
}

#close:hover{
  /* background-color: rgb(201, 59, 59); */
  background-color: #9e3b3b;
}

footer{
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  /* background: #323233; */
  color: rgba(255, 255, 255, 0.534);
  font-size: 0.85em;
  padding: 3px;
  text-align: right;
}

</style>
