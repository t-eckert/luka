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