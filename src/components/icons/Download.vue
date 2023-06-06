<template>
    <div class="icon">
        <svg
        :width="width"
            :height="height"
            stroke-width="1.5"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            :color="color"
        >
            <path
                d="M6 20h12M12 4v12m0 0l3.5-3.5M12 16l-3.5-3.5"
                :stroke="color"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            ></path>
        </svg>
    </div>
</template>
<script lang="ts">
export default {
    name: "Download",
    props: {
        color: {
            type: String,
            default: "#fff",
        },
        width: {
            type: String,
            default: "24px",
        },
        height: {
            type: String,
            default: "24px",
        },
    },
};
</script>

<style scoped lang="less"></style>
