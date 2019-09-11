<template>
  <button>Push</button>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class ButtonPush extends Vue {}
</script>

<style scoped>
button {
  background-color: var(--button-push);
}

button:hover {
  background-color: var(--button-push-hover);
}

button:active {
  background-color: var(--button-push);
}
</style>