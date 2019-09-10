<template>
  <div class="calculator-base">
    <ButtonOperation operation="+" />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import ButtonOperation from "./buttons/ButtonOperation.vue";

@Component({
  components: {
    ButtonOperation
  }
})
export default class CalculatorBase extends Vue {}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
