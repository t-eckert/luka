<template>
  <div class="calculator">
    <BaseState :state="state" />
    <BaseOperations />
    <BaseNumbers />
    <BaseActions />
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
export default class Calculator extends Vue {
  state: number[] = [10000100200, 2, 0.0030043, 4.0];
}
</script>

<style scoped lang="scss">
</style>
