<template>
  <div class="numbers">
    <div class="number-row">
      <ButtonNumber number="7" />
      <ButtonNumber number="8" />
      <ButtonNumber number="9" />
    </div>
    <div class="number-row">
      <ButtonNumber number="4" />
      <ButtonNumber number="5" />
      <ButtonNumber number="6" />
    </div>
    <div class="number-row">
      <ButtonNumber number="1" />
      <ButtonNumber number="2" />
      <ButtonNumber number="3" />
    </div>
    <div class="number-row">
      <ButtonNumber number="0" />
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import ButtonNumber from "../buttons/ButtonNumber.vue";

@Component({
  components: { ButtonNumber }
})
export default class BaseNumbers extends Vue {}
</script>