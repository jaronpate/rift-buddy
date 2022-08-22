<template>
  <div class="modal">
    <div class="background" @click="$emit('close')"></div>
    <div class="content flex-center flex-col">
      <h2>{{ title }}</h2>
      <slot></slot>
    </div>
  </div>
</template>

<script lang="ts">
  export default {
    name: 'Modal',
    props: ['title']
  }
</script>

<style scoped lang="less">
@primary: #b39556;

.mx-10{
    margin: 10px 0;
}

.mx-20{
    margin: 20px 0;
}

.flex-col{
    flex-direction: column;
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
.modal {
    position: absolute;
    top: 0;
    left: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 100;
    height: 100%;
    width: 100%;

    .background {
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        width: 100%;
        background-color: fade(#111, 80%);
    }

    .content {
        width: 50%;
        height: 30%;
        padding: 0 20px;
        border-radius: 5px;
        background-color: #111;
        z-index: 10;

        h1, h2, h3 {
            color: @primary;
            margin-top: 0;
        }

        select, .search, input{
            width: 90%;
        }
        
        .options {
            margin-top: 20px;
        }
    }
}
</style>
