<template>
    <div class="main">
        <header data-tauri-drag-region class="flex-center connection">
            <div data-tauri-drag-region class="handle"></div>
            <div class="user flex-center">
                <template v-if="user">
                    <!-- <h3>Rift Buddy</h3> -->
                    <div class="pfp flex-center">
                        <img :src="`https://ddragon.leagueoflegends.com/cdn/12.15.1/img/profileicon/${user.profileIconId}.png`" />
                    </div>
                    <div class="username">
                        <em class="sub-text">Logged in as {{ user.displayName }}</em>
                    </div>
                </template>
                <template v-else>
                    <div class="pfp flex-center">
                        <img src="https://ddragon.leagueoflegends.com/cdn/12.15.1/img/item/1001.png" />
                    </div>
                    <div class="username"><em class="sub-text">LOL client not found...</em></div>
                </template>
            </div>
            <div class="ff"></div>
            <!-- <div class="tabs">
                <div class="tab" :class="{ selected: tab === 'runes' }" @click="tab = 'runes'">Runes</div>
                <div class="tab" :class="{ selected: tab === 'champ' }" @click="tab = 'champ'">Champ Selection</div>
            </div> -->
            <div class="ff"></div>
            <div class="nav-btns">
                <svg class="nav-btn" @click="() => minimize()" aria-hidden="false" width="12" height="12" viewBox="0 0 12 12">
                    <rect fill="currentColor" width="10" height="1" x="1" y="6"></rect>
                </svg>
                <svg class="nav-btn" @click="() => toggleMaximize()" aria-hidden="false" width="12" height="12" viewBox="0 0 12 12">
                    <rect width="9" height="9" x="1.5" y="1.5" fill="none" stroke="currentColor"></rect>
                </svg>
                <svg class="nav-btn" @click="() => close()" id="close" aria-hidden="false" width="12" height="12" viewBox="0 0 12 12">
                    <polygon
                        fill="currentColor"
                        fill-rule="evenodd"
                        points="11 1.576 6.583 6 11 10.424 10.424 11 6 6.583 1.576 11 1 10.424 5.417 6 1 1.576 1.576 1 6 5.417 10.424 1"
                    ></polygon>
                </svg>
            </div>
        </header>
        <div class="content">
            <div class="tab-nav">
                <!-- <div class="user" v-if="user">
                    <div class="pfp flex-center">
                        <img :src="`https://ddragon.leagueoflegends.com/cdn/12.15.1/img/profileicon/${user.profileIconId}.png`" />
                    </div>
                    <div class="username">{{ user.displayName }}</div>
                </div> -->
                <div class="tab-nav-btn" :class="{ active: tab === _tab.is }" v-for="(_tab, name) in tabs" @click="() => (tab = _tab.is)">
                    <div class="icon">
                        <component
                            :is="_tab.icon"
                            :color="tab === _tab.is ? '#b39556' : 'white'"
                            height="18px"
                            width="18px"
                            stroke-width="2"
                        />
                    </div>
                    <div>{{ _tab.name }}</div>
                </div>
            </div>
            <div class="tab ff">
                <component :is="tab" />
            </div>
        </div>
        <div class="toast" :class="{ [toast.type]: !!toast.type }" v-if="toast.show">
            {{ toast.message }}
        </div>
    </div>
</template>

<script>
// import axios from "axios";
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { SaveFloppyDisk, AddCircle, Import, Settings } from "@iconoir/vue";
import SavedPages from "@/components/pages/SavedPages.vue";
import CreatePage from "@/components/pages/CreatePage.vue";
import ImportPage from "@/components/pages/ImportPage.vue";

export default {
    name: "App",
    components: {},
    data: () => {
        return {
            user: null,
            tab: SavedPages,
            tabs: {
                saved: {
                    name: "Saved Pages",
                    icon: SaveFloppyDisk,
                    is: SavedPages,
                },
                create: {
                    name: "Create New",
                    icon: AddCircle,
                    is: CreatePage,
                },
                import: {
                    name: "Import",
                    icon: Import,
                    is: ImportPage,
                },
                // settings: {
                //     name: "Settings",
                //     icon: Settings,
                //     is: null,
                // },
            },
            connected: false,
            timeout: null,
            toast: {
                show: false,
                message: null,
            },
        };
    },
    provide() {
        return {
            connected: computed(() => this.connected),
        };
    },
    mounted() {
        this.getClientInfo();

        this.$bus.on("app.tab", (tab) => {
            this.tab = tab;
        });

        this.$bus.on("app.close", () => {
            this.close();
        });

        this.$bus.on("app.toast", (toast) => {
            this.toast = {
                show: true,
                ...toast,
            };
            setTimeout(() => {
                this.toast = { show: false, message: null };
            }, 3000);
        });
    },
    watch: {
        connected(val) {
            if (val) {
                invoke("lcu", { endpoint: "/lol-summoner/v1/current-summoner", method: "GET" })
                    .then((res) => {
                        console.log(res);
                        this.user = res;
                    })
                    .catch((err) => {
                        console.log(err);
                    });
            } else {
                this.user = null;
            }
        },
    },
    methods: {
        getClientInfo(timeout = 0) {
            clearTimeout(this.timeout);
            this.timeout = setTimeout(() => {
                invoke("get_client_info")
                    .then((res) => {
                        // console.log(res);
                        this.connected = true;
                        this.getClientInfo(5000);
                    })
                    .catch((err) => {
                        console.log(err);
                        this.connected = false;
                        this.getClientInfo(1000);
                    });
            }, timeout);
        },
        minimize() {
            appWindow.minimize();
        },
        toggleMaximize() {
            appWindow.toggleMaximize();
        },
        close() {
            appWindow.close();
        },
    },
};
</script>

<style lang="less">
body {
    margin: 0;
}
</style>

<style scoped lang="less">
@import "@/assets/css/main.less";

.main {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

.content {
    display: flex;
    flex: 1;
    flex-direction: row;
    overflow: hidden;
}

.tab {
    flex: 1;
    padding: 15px;
    overflow: auto;

    &::-webkit-scrollbar {
        width: 5px;
    }

    &::-webkit-scrollbar-track {
        background: #222;
    }

    &::-webkit-scrollbar-thumb {
        background: #b39556;
    }

    &::-webkit-scrollbar-thumb:hover {
        background: #b39556;
    }
}

.toast {
    position: fixed;
    bottom: 15px;
    right: 15px;
    padding: 10px;
    background-color: #222;
    border-radius: 5px;
    color: white;
    font-size: 14px;
    line-height: 16px;
    font-weight: 600;
    z-index: 100;
    animation: slideIn 0.5s ease-in-out;
    max-width: 400px;

    &.error {
        background-color: #b54e4e;
    }

    @keyframes slideIn {
        0% {
            transform: translateY(100%);
        }
        100% {
            transform: translateY(0);
        }
    }
}

.tab-nav {
    background-color: #222;
    width: 150px;

    .user {
        padding: 15px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background-color: #141414;

        .pfp {
            width: 70px;
            height: 70px;
            border-radius: 50%;
            overflow: hidden;
            border: #222222 solid 3px;
            background-color: fade(#111, 30%);
            margin-bottom: 5px;

            img {
                width: 100%;
                height: 100%;
                object-fit: cover;
            }
        }

        .username {
            font-size: 16px;
            font-weight: 600;
        }
    }
}

.tab-nav-btn {
    display: flex;
    align-items: center;
    padding: 6px 10px;
    font-size: 14px;
    font-weight: 600;
    // color: @primary;

    .icon {
        margin-right: 7px;
    }

    &.active {
        background-color: #2f2f2f;
        color: #b39556;
    }

    &:hover {
        cursor: pointer;
        background-color: fade(#141414, 35%);

        &.active {
            background-color: #2f2f2f;
        }
    }
}

.nav-btns {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    height: 40px;
}

.nav-btn {
    height: 100%;
    padding: 0 11px;
    height: 35px;
    width: 35px;

    &:hover {
        cursor: pointer;
        background-color: fade(#333, 45%);
    }

    &#close:hover {
        background-color: fade(#ff3d33, 65%);
    }
}

header {
    padding: 3px 15px;

    &::selection {
        background-color: transparent;
    }
}

header.connection {
    padding: 5px 11px;
    padding-right: 0;
    height: 35px;
    background-color: #111;
    // background-color: fade(#111, 80%);
    // background-color: fade(#111, 50%);
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
                content: "";
                position: absolute;
                transform: scaleX(0);
                width: 100%;
                height: 2px;
                bottom: 3px;
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
        pointer-events: none;
        .pfp {
            img {
                border: #222 solid 2px;
                background-color: fade(#111, 30%);
                border-radius: 999px;
                height: 25px;

                &::selection {
                    background-color: transparent;
                }
            }
        }

        .username {
            white-space: nowrap;
            margin-left: 6px;
            font-size: 18px;
            line-height: 21px;
            .sub-text {
                font-size: 12px;

                &::selection {
                    background-color: transparent;
                }
            }
        }
    }
}
</style>
