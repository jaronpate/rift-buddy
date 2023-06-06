<template>
    <div class="icon">
        <svg
            :width="width"
            :height="height"
            stroke-width="2"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            :color="color"
        >
            <path
                d="M19.141 5A9.97 9.97 0 0012 2C6.477 2 2 6.477 2 12a9.968 9.968 0 002.859 7M19.14 5A9.967 9.967 0 0122 12c0 5.523-4.477 10-10 10a9.97 9.97 0 01-7.141-3M19.14 5L4.86 19"
                :stroke="color"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            ></path>
        </svg>
    </div>
</template>
<script lang="ts">
export default {
    name: "Prohibition",
    props: {
        color: {
            type: String,
            default: "#fff"
        },
        width: {
            type: String,
            default: "24px"
        },
        height: {
            type: String,
            default: "24px"
        },
    },
};
</script>

<style scoped lang="less"></style>
