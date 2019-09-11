<template>
  <div class="actions">
    <ButtonPush />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import ButtonPush from "../buttons/ButtonPush.vue";

@Component({
  components: {
    ButtonPush
  }
})
export default class BaseActions extends Vue {}
</script>