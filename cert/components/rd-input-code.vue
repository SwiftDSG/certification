<template>
  <div class="rd-input-component">
    <input
      type="text"
      id="rd-input-code"
      class="rd-input"
      ref="rdInput"
      @focusout="focusEnabled = false"
      @focusin="assignFocus"
      @input="updateModel"
      :disabled="input.disabled"
    />
    <label for="rd-input-code" class="rd-input-area">
      <div
        v-for="n in input.length"
        :key="n"
        class="rd-input-area-field"
        :style="`width: calc((100% - ${(input.length - 1) * 0.75}rem) / ${
          input.length
        })`"
        :class="
          focusEnabled && focusIndex === n - 1
            ? 'rd-input-area-field-focus'
            : ''
        "
      >
        <span class="rd-input-area-field-text rd-headline-3">{{
          input.model[n - 1]
        }}</span>
        <div class="rd-input-area-field-border"></div>
      </div>
    </label>
  </div>
</template>

<script lang="ts" setup>
  import { InputCodeOption } from "~~/types/general";

  const props = defineProps<{
    input: InputCodeOption;
  }>();

  const rdInput = ref<HTMLInputElement | null>(null);

  const inputModel = ref<string>("");

  const focusIndex = ref<number>(0);
  const focusEnabled = ref<boolean>(false);

  function assignFocus(): void {
    focusEnabled.value = true;
    if (!props.input.model.length) focusIndex.value = 0;
    else {
      focusIndex.value =
        props.input.model.length >= props.input.length
          ? props.input.length - 1
          : props.input.model.length - 1;
    }
  }

  function updateModel(e: Event): void {
    if (e.target instanceof HTMLInputElement) {
      e.target.value = e.target.value.toUpperCase();
      if (e.target.value.length <= props.input.length) {
        inputModel.value = e.target.value;
        props.input.model = e.target.value;
        assignFocus();
      } else {
        e.target.value = e.target.value.slice(0, props.input.length);
      }
    }
  }
</script>

<style lang="scss" scoped>
  .rd-input-component {
    position: relative;
    display: flex;
    flex-direction: column;
    input.rd-input {
      position: absolute;
      opacity: 0;
    }
    label.rd-input-area {
      position: relative;
      width: 100%;
      height: 2rem;
      display: flex;
      justify-content: space-between;
      .rd-input-area-field {
        pointer-events: none;
        position: relative;
        height: 100%;
        background: var(--background-depth-three-color);
        border-radius: 0.5rem;
        display: flex;
        transition: background-color 0.25s;
        span.rd-input-area-field-text {
          position: relative;
          width: 100%;
          height: 100%;
          text-transform: uppercase;
          display: flex;
          justify-content: center;
          align-items: center;
        }
        .rd-input-area-field-border {
          pointer-events: none;
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          border-radius: 0.5rem;
          border: 1px solid rgba(0, 0, 0, 0.125);
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
        &.rd-input-area-field-focus {
          background: var(--background-depth-two-color);
          .rd-input-area-field-border {
            border-color: var(--primary-color);
            &::before {
              opacity: 0.25;
            }
          }
        }
      }
    }
  }
</style>
