<template>
  <button>{{ number }}</button>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class ButtonNumber extends Vue {
  @Prop() private number!: string;
}
</script>