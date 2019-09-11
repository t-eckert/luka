<template>
  <div class="state">
    <div class="stack-value">{{ state[state.length - 4] || "" }}</div>
    <div class="stack-value">{{ state[state.length - 3] || "" }}</div>
    <div class="stack-value">{{ state[state.length - 2] || "" }}</div>
    <div class="stack-value">{{ state[state.length - 1] || "" }}</div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

@Component({
  components: {}
})
export default class BaseState extends Vue {
  @Prop() private state!: number[];
}
</script>