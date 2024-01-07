<template>
  <div class="rd-input-component" :class="input.disabled ? 'rd-input-disabled' : ''">
    <input v-for="(option, i) in input.options" :key="option.value" type="radio" :name="input.name" :id="`${id}-${i}`"
      class="rd-input" ref="rdInput" :disabled="input.disabled" @change="updateModel(i)" />
    <div class="rd-input-border"></div>
    <div class="rd-input-slider" :style="`left: calc(${translate * index * 100}% + 6px); width: calc(${translate * 100
      }% - ${input.options.length === 2 ? 6 : 12}px);`"></div>
    <div class="rd-input-options">
      <label v-for="(option, i) in input.options" :key="option.value" :for="`${id}-${i}`"
        class="rd-input-option rd-body-text" :class="index === i ? 'rd-input-option-active' : ''"
        @keypress.left.up="updateModel(i === 0 ? input.options.length - 1 : i - 1)"
        @keypress.right.down="updateModel(i === input.options.length - 1 ? 0 : i + 1)">{{ option.name }}</label>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { InputSwitchOption } from "~~/types/general";

const props = defineProps<{
  input: InputSwitchOption;
}>();

const rdInput = ref<HTMLInputElement | null>(null);

const translate = computed<number>(() =>
  props.input.options.length === 3 ? 1 / 3 : 1 / 2
);
const index = computed<number>(() =>
  props.input.options.findIndex((a) => a.value === props.input.model.value)
);
const id = computed<string>(() => {
  const characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let str = "";
  for (let i = 0; i < 10; i++) {
    str += characters[Math.round(Math.random() * (characters.length - 1))];
  }

  return `rd-input-${str}`;
});

function updateModel(index: number): void {
  props.input.model = props.input.options[index];
}

watch(
  () => props.input.model,
  (val) => {
    if (rdInput.value) {
      if (val === props.input.options[0]) rdInput.value.checked = false;
      else rdInput.value.checked = true;
    }
  }
);

onMounted(() => {
  if (!props.input.model.value) {
    props.input.model = props.input.options[0];
  }
});
</script>

<style lang="scss" scoped>
.rd-input-component {
  position: relative;
  width: 100%;
  height: 2rem;
  display: flex;
  align-items: center;

  input.rd-input {
    pointer-events: none;
    position: absolute;
    opacity: 0;
  }

  .rd-input-border {
    pointer-events: none;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: 0.5rem;
    border: var(--border);
    background: var(--background-depth-one-color);
    box-sizing: border-box;
    transition: 0.25s border-color, 0.25s border-width;

    &::before {
      content: "";
      position: absolute;
      top: -3px;
      left: -3px;
      width: calc(100% + 6px);
      height: calc(100% + 6px);
      border-radius: 0.5rem;
      border: 3px solid var(--primary-color);
      box-sizing: border-box;
      opacity: 0;
      transition: 0.25s opacity;
    }
  }

  .rd-input-slider {
    position: absolute;
    top: 6px;
    left: 6px;
    width: calc(50% - 12px);
    height: calc(100% - 12px);
    background: var(--primary-color);
    border-radius: calc(0.5rem - 4px);
    transition: left 0.25s;
  }

  .rd-input-options {
    position: relative;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;

    label.rd-input-option {
      cursor: pointer;
      position: relative;
      width: 100%;
      height: 100%;
      color: var(--font-primary-color);
      opacity: 0.5;
      display: flex;
      justify-content: center;
      align-items: center;
      transition: opacity 0.25s, color 0.25s;

      &:focus {
        outline: none;
        opacity: 1;
        color: var(--primary-color);
      }

      &.rd-input-option-active {
        opacity: 1 !important;
        color: var(--font-secondary-color) !important;
      }

      &:hover {
        opacity: 1;
      }
    }
  }

  &.rd-input-disabled {
    pointer-events: none;
    opacity: 0.5;
  }

  &:focus-within {
    .rd-input-border {
      border-color: var(--primary-color);

      &::before {
        opacity: 0.25;
      }
    }
  }
}
</style>
