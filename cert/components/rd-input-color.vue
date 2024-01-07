<template>
  <div class="rd-component">
    <div class="rd-component-input-wrapper">
      <rd-input-text
        :input="colorInput"
        class="rd-component-input"
      ></rd-input-text>
      <rd-input-text :input="alphaInput" class="rd-component-input" />
    </div>
  </div>
</template>

<script lang="ts" setup>
  import { ComputedRef } from "vue";
  import { InputColorOption, InputOption } from "~~/types/general";

  const props = defineProps<{
    input: InputColorOption;
  }>();

  const colorInput = ref<InputOption>({
    name: "color",
    label: "Color",
    placeholder: "",
    model: "ffa84c",
    color: "#ffa84c",
  });
  const alphaInput = ref<InputOption>({
    name: "alpha",
    type: "number",
    placeholder: "",
    model: "",
  });

  const color: ComputedRef<string> = computed(() => colorInput.value.model);
  const alpha: ComputedRef<number> = computed(() => {
    let num: number = parseInt(alphaInput.value.model.split(".").join(""));
    if (num > 100) num = 100;
    else if (num < 0) num = 0;
    return num;
  });

  watch(
    () => color.value,
    (val) => {
      if (val) colorInput.value.color = `#${val}`;
      else colorInput.value.color = "#000";
      props.input.color = colorInput.value.model;
    }
  );
  watch(
    () => alpha.value,
    (val) => {
      props.input.alpha = val;
    }
  );
  watch(
    () => props.input,
    (val) => {
      if (val) {
        colorInput.value.model = val.color;
        colorInput.value.color = `#${val.color}`;
        alphaInput.value.model = val.alpha.toString();
      }
    },
    { deep: true, immediate: true }
  );
</script>

<style lang="scss" scoped>
  .rd-component {
    position: relative;
    width: 100%;
    display: flex;
    .rd-component-input-wrapper {
      position: relative;
      width: 100%;
      display: flex;
      justify-content: space-between;
      align-items: flex-end;
      .rd-component-input {
        &:first-child {
          width: calc(100% - 4.75rem);
        }
        &:last-child {
          width: 4rem;
        }
      }
    }
  }
</style>
