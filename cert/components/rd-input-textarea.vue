<template>
  <div
    ref="rdInputComponent"
    class="rd-input-component"
    :class="`${input.error ? 'rd-input-error' : ''} ${
      input.disabled ? 'rd-input-disabled' : ''
    }`"
  >
    <label v-if="input.label" class="rd-input-label rd-headline-6">{{
      input.label
    }}</label>
    <div class="rd-input-container">
      <textarea
        class="rd-input rd-body-text"
        type="text"
        :placeholder="input.placeholder"
        :name="input.name"
        :disabled="input.disabled"
        ref="rdInput"
        @input="updateModel"
        @focusout="focusHandler"
      ></textarea>
      <div class="rd-input-border"></div>
    </div>
    <span
      v-if="typeof input.error === 'string'"
      class="rd-input-error rd-caption-text"
    >
      <span class="rd-text-wrapper">
        <span class="rd-text-container rd-text-container-up">
          <span class="rd-text">{{ inputError }}</span>
        </span>
      </span>
    </span>
  </div>
</template>

<script lang="ts" setup>
  import { InputOption } from "~~/types/general.js";

  const props = defineProps<{
    input: InputOption;
  }>();
  const emits = defineEmits(["focusout"]);

  const rdInput = ref<HTMLInputElement | null>(null);

  const inputError = ref<string | undefined>(props.input.error);
  const inputModel = ref<string>("");

  function focusHandler(): void {
    emits("focusout", props.input);
  }
  function updateModel(e: Event): void {
    if (e.target instanceof HTMLInputElement) {
      if (props.input.type === "number" && e instanceof InputEvent) {
        if ((e.data && "1234567890".includes(e.data)) || !e.data) {
          const rawValue = parseInt(e.target.value.split(".").join(""));
          const value = rawValue ? rawValue.toLocaleString("de-DE") : "";
          e.target.value = value;
          inputModel.value = value;
          props.input.model = {
            name: value,
            value: rawValue,
          };
        } else {
          e.target.value = inputModel.value;
        }
      } else {
        inputModel.value = e.target.value;
        props.input.model = {
          name: e.target.value,
          value: e.target.value,
        };
      }
    }
  }

  watch(
    () => props.input.model,
    (val) => {
      if (inputModel.value !== val.name && rdInput.value) {
        rdInput.value.value = val.name;
        inputModel.value = val.name;
      }
    },
    { deep: true }
  );
  watch(
    () => props.input.error,
    (val) => {
      if (val) inputError.value = val;
    }
  );

  onMounted(() => {
    if (props.input.model.name && rdInput.value) {
      rdInput.value.value = props.input.model.name;
      inputModel.value = props.input.model.name;
    }
  });
</script>

<style lang="scss" scoped>
  .rd-input-component {
    position: relative;
    display: flex;
    flex-direction: column;
    label.rd-input-label {
      position: relative;
      width: 100%;
      height: 1rem;
      display: flex;
      color: var(--font-primary-color);
      opacity: 0.5;
      align-items: center;
    }
    .rd-input-container {
      position: relative;
      width: 100%;
      height: 5rem;
      background: var(--background-depth-one-color);
      border-radius: 0.5rem;
      display: flex;
      align-items: center;
      textarea.rd-input {
        resize: none;
        position: relative;
        width: 100%;
        height: 100%;
        padding: 0.5rem;
        border: none;
        border-radius: 0.5rem;
        box-sizing: border-box;
        color: var(--font-primary-color);
        background: rgba(0, 0, 0, 0);
        display: flex;
        transition: background-color 0.25s;
        &::placeholder {
          color: var(--font-primary-color);
          opacity: 0.5;
          transition: opacity 0.25s;
        }
        &:hover {
          outline: none;
          background: rgba(0, 0, 0, 0);
          &::placeholder {
            opacity: 1;
          }
        }
        &:focus {
          outline: none;
          background: var(--background-depth-two-color);
          &::placeholder {
            opacity: 1;
          }
        }
        &:focus + .rd-input-border {
          border-color: var(--primary-color);
          &::before {
            opacity: 0.25;
          }
        }
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
        box-sizing: border-box;
        transition: 0.25s border-color, 0.25s border-width;
        &::before {
          content: "";
          position: absolute;
          top: -3px;
          left: -3px;
          width: calc(100% + 6px);
          height: calc(100% + 6px);
          border-radius: calc(0.5rem + 1px);
          border: 3px solid var(--primary-color);
          box-sizing: border-box;
          opacity: 0;
          transition: 0.25s opacity;
        }
      }
    }
    span.rd-input-error {
      position: relative;
      width: 100%;
      height: 1rem;
      display: flex;
      color: var(--error-color);
      align-items: center;
      span.rd-text-wrapper {
        span.rd-text-container {
          transition: 0.25s transform;
          span.rd-text {
            transition: 0.25s transform;
          }
        }
      }
    }
    &.rd-input-error {
      span.rd-input-error {
        span.rd-text-wrapper {
          span.rd-text-container {
            transform: translateY(0);
            span.rd-text {
              transform: translateY(0);
            }
          }
        }
      }
    }
    &.rd-input-disabled {
      pointer-events: none;
      filter: grayscale(0.75);
      opacity: 0.5;
    }
  }
</style>
