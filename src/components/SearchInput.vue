<!-- <template>
    <div class="search">
        <input type="text" :class="{ 'not-loading': (results.length < 1 || !show_results) }" v-model="query"
            @keyup="find" @focus="show_results = true" @blur="() => close_results()" :placeholder="placeholder" />
        <div class="results" v-if="results.length && show_results">
            <template v-if="loading">
                <div class="loading"></div>
            </template>
            <slot></slot>
        </div>
    </div>
</template>

<script lang="ts">
export default {
    name: 'SearchInput',
    props: ['placeholder', 'results'],
    data() {
        return {
            show_results: false,
            loading: false
        }
    },
    methods: {
        close_results() {
            const _this = this;
            setTimeout(() => _this.show_results = false, 300);
        }
    }
}
</script>

<style scoped>
</style> -->
