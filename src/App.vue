<template>
  <div>
    
    <div id="app" style="margin-top: 15px">
      <div class="profile">
        <img v-bind:src="summoner.icon" style="max-height: 64px; border-radius: 50%;">
        <h1>{{ summoner.displayName }}</h1>
      </div>
      <!-- <img v-bind:src="champion.icon" style="max-height: 64px;">
      <h1>{{ champion.name }}</h1> -->
    </div>
    <p>{{version}}</p>

  </div>
</template>

<script>
import riot from './lib/riotApi.js'
const invoke = window.__TAURI__.invoke
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
      splash: ""
    }
  },
  mounted: function(){
  },
  asyncComputed: {
    async summoner() {
      let sum = await invoke('get_credentials').then(async (connData) => {
        return await invoke('lcu', {
          endpoint: '/lol-summoner/v1/current-summoner',
          port: connData.port,
          password: connData.password
        }).then((data) => data)
      });
      sum.icon = `http://ddragon.leagueoflegends.com/cdn/${ddVer}/img/profileicon/${sum.profileIconId}.png`
      return sum;
    },
    async version() {
      return await riot("get", "/realms/na.json")
        .then(data => data.v)
    },
    async champions() {
      return await riot("get", `/cdn/${"10.21.1"}/data/en_US/champion.json`)
        .then((res) => res.data)
    }
  },
  watch: {
    summoner(data) {
      console.log(data)
    },
    version(vv){
      console.log(vv)
      ddVer = vv
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
}

.profile img{
  border: 3px solid rgba(128, 128, 128, 0.35);
  padding: 3px;
}

.profile h1{
  margin-left: 10px;
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
  height: 13px;
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

</style>
