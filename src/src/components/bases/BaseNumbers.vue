<template>
  <div class="numbers"></div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import ButtonNumber from "../buttons/ButtonNumber.vue";


@Component({
  components: {}
})
export default class BaseState extends Vue {}
</script>