<template>
  <button>{{ operation }}</button>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class ButtonOperation extends Vue {
  @Prop() private operation!: string;
}
</script>

<style scoped>
:root {
  --button-default: #38b2ac;
  --button-hover: #319795;
}

button {
  background-color: var(--button-default);
}

button:hover {
  background-color: var(--button-hover);
}

button:active {
  background-color: var(--button-default);
}
</style>