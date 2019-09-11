<template>
  <div class="calculator">
    <BaseNumbers />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import BaseActions from "./bases/BaseActions.vue";
import BaseNumbers from "./bases/BaseNumbers.vue";
import BaseOperations from "./bases/BaseOperations.vue";
import BaseState from "./bases/BaseState.vue";

@Component({
  components: {
    BaseActions,
    BaseNumbers,
    BaseOperations,
    BaseState
  }
})
export default class Calculator extends Vue {}
</script>

<style scoped lang="scss">
</style>
