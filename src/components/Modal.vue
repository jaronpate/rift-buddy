<template>
    <div class="modal">
        <div class="background" @click="$emit('close')"></div>
        <div class="content flex-center flex-col">
            <div class="wrapper" :class="{'waiting': loading}">
                <h2>{{ title }}</h2>
                <slot></slot>
            </div>
            <div class="loading-modal" v-if="loading"></div>
        </div>
    </div>
</template>

<script lang="ts">
export default {
    name: "Modal",
    props: {
        title: {
            type: String,
            default: "Modal"
        },
        loading: {
            type: Boolean,
            default: false
        }
    },
};
</script>

<style scoped lang="less">
@import '../assets/css/main.less';
</style>
