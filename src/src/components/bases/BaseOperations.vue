<template>
  <div class="operations">
    <ButtonOperation operation="+" />
    <ButtonOperation operation="-" />
    <ButtonOperation operation="*" />
    <ButtonOperation operation="/" />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import ButtonOperation from "../buttons/ButtonOperation.vue";

@Component({
  components: {
    ButtonOperation
  }
})
export default class BaseOperations extends Vue {}
</script>