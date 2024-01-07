<template>
  <div
    ref="rdInputComponent"
    class="rd-input-component"
    :class="`${input.error ? 'rd-input-error-active' : ''} ${
      input.disabled ? 'rd-input-disabled' : ''
    }`"
  >
    <label v-if="input.label" class="rd-input-label rd-headline-6">{{
      input.label
    }}</label>
    <div class="rd-input-container">
      <div v-if="input.icon" class="rd-input-icon-container">
        <rd-svg :name="input.icon" :color="'secondary'" />
      </div>
      <input
        class="rd-input rd-body-text"
        :placeholder="input.placeholder"
        ref="rdInput"
        :name="input.name"
        :type="input.type"
        @focus="dropDownHandler('open')"
        @blur="selectOption(true)"
        @keydown.up.prevent="
          selectIndex(
            dropDownIndex <= 0
              ? (input.options?.length || 0) - 1
              : dropDownIndex - 1
          )
        "
        @keydown.down.prevent="
          selectIndex(
            dropDownIndex >= (input.options?.length || 0) - 1
              ? 0
              : dropDownIndex + 1
          )
        "
        @keypress.enter="selectOption(false)"
        @input="updateModel"
        :disabled="input.disabled"
      />
      <div class="rd-input-border"></div>
      <div v-if="inputOptions?.length" class="rd-input-chevron-container">
        <rd-svg name="chevron-down" />
      </div>
      <div
        v-if="inputOptions?.length"
        ref="rdInputOptions"
        class="rd-input-options"
        :class="dropDownOpened ? 'rd-input-options-opened' : ''"
      >
        <div
          class="rd-input-option rd-body-text"
          v-for="(option, i) in inputOptions"
          :key="i"
          @mouseover="selectIndex(i)"
          @mousedown="selectOption(false)"
          :class="dropDownIndex === i ? 'rd-input-option-selected' : ''"
        >
          {{ typeof option === "string" ? option : option.name }}
        </div>
      </div>
    </div>
    <span
      v-if="typeof input.error === 'string'"
      class="rd-input-error rd-headline-6"
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
  import gsap from "gsap";
  import { InputGeneric, InputOption } from "~~/types/general.js";

  const props = defineProps<{
    input: InputOption | InputGeneric<any>;
  }>();

  const rdInputOptions = ref<HTMLDivElement | null>(null);
  const rdInput = ref<HTMLInputElement | null>(null);

  const inputError = ref<string | undefined>(props.input.error);
  const inputModel = ref<string>("");
  const inputValue = ref<string>("");
  const inputOptions = ref<InputOption["options"]>(props.input.options);

  const dropDownAnim = ref<GSAPAnimation | null>(null);
  const dropDownOpened = ref<boolean>(false);
  const dropDownIndex = ref<number>(-1);

  const animate = {
    dropDownHandler(
      rdInputOptions: HTMLElement,
      cb?: () => void
    ): GSAPTimeline {
      const tl: GSAPTimeline = gsap.timeline({
        paused: true,
        onComplete() {
          if (cb) cb();
        },
      });

      tl.to(rdInputOptions, {
        scale: 1,
        opacity: 1,
        duration: 0.25,
        ease: "power2.inOut",
      });

      return tl;
    },
  };

  function dropDownHandler(state: "open" | "close"): void {
    if (!dropDownAnim.value && rdInputOptions.value)
      dropDownAnim.value = animate.dropDownHandler(rdInputOptions.value, () => {
        dropDownOpened.value = true;
      });
    if (dropDownAnim.value) {
      if (state === "open") {
        dropDownIndex.value = -1;
        dropDownAnim.value.play();
      } else {
        setTimeout(() => {
          if (dropDownOpened.value && dropDownAnim.value) {
            dropDownOpened.value = false;
            dropDownIndex.value = -1;
            dropDownAnim.value.reverse();
          }
        }, 50);
      }
    }
  }

  function updateModel(e: Event): void {
    if (e.target instanceof HTMLInputElement) {
      inputModel.value = e.target.value;
      props.input.model = inputModel.value;
      filterOptions(e.target.value);
      dropDownHandler("open");
    }
  }

  function filterOptions(str: string): void {
    if (str.length) {
      inputOptions.value = [...(props.input.options || [])].filter((a) =>
        (typeof a === "string" ? a : a.name)
          .replace(/[^A-Za-z0-9]/g, "")
          .toLowerCase()
          .includes(str.replace(/[^A-Za-z0-9]/g, "").toLowerCase())
      );
    } else {
      inputOptions.value = props.input.options;
    }
  }

  function selectIndex(index: number): void {
    dropDownIndex.value = index;
  }

  function selectOption(clear?: boolean): void {
    if (clear) dropDownIndex.value = -1;
    if (inputOptions.value && rdInput.value) {
      const selection =
        dropDownIndex.value > -1
          ? inputOptions.value[dropDownIndex.value]
          : inputOptions.value.find(
              (a) =>
                (typeof a === "string" ? a : a.name)
                  .replace(/[^A-Za-z0-9]/g, "")
                  .toLowerCase() ===
                inputModel.value.replace(/[^A-Za-z0-9]/g, "").toLowerCase()
            );
      if (selection) {
        inputModel.value =
          typeof selection === "string" ? selection : selection.name;
        inputValue.value =
          typeof selection === "string" ? selection : selection.value;
        props.input.model = inputModel.value;
        props.input.value = inputValue.value;
        rdInput.value.value = inputModel.value;
        dropDownHandler("close");
        setTimeout(() => {
          filterOptions(inputModel.value);
        }, 250);
      } else {
        inputValue.value = "";
        props.input.value = inputValue.value;
        dropDownHandler("close");
      }
    }
  }

  watch(
    () => props.input.error,
    (val) => {
      if (val) inputError.value = val;
    }
  );
  watch(
    () => props.input.model,
    (val) => {
      if (inputModel.value !== val && rdInput.value && inputOptions.value) {
        rdInput.value.value = val;
        inputModel.value = val;
        const index: number = inputOptions.value.findIndex(
          (a) => (typeof a === "string" ? a : a.name) === val
        );
        if (index > -1) {
          dropDownIndex.value = index;
          selectOption();
        } else {
          inputValue.value = val;
        }
      }
    }
  );
  watch(
    () => props.input.options,
    (val) => {
      inputOptions.value = val;
    },
    {
      deep: true,
    }
  );
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
      height: 2rem;
      background: var(--background-depth-one-color);
      border-radius: 0.5rem;
      display: flex;
      align-items: center;

      .rd-input-icon-container {
        position: relative;
        width: 2rem;
        height: 2rem;
        padding: 0 0.5rem;
        border-top-left-radius: 0.5rem;
        border-bottom-left-radius: 0.5rem;
        box-sizing: border-box;
        background: rgba(0, 0, 0, 0.05);
        display: flex;
        justify-content: center;
        align-items: center;
      }

      input.rd-input {
        position: relative;
        width: 100%;
        height: 100%;
        padding: 0 2rem 0 0.5rem;
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
        }
        &:focus + .rd-input-border {
          border-color: var(--primary-color);
          &::before {
            opacity: 0.25;
          }
        }
        &:focus ~ .rd-input-chevron-container {
          rotate: 180deg;
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

      .rd-input-options {
        z-index: 20000;
        pointer-events: none;
        position: absolute;
        top: 100%;
        left: 0;
        width: 100%;
        max-height: 9rem;
        background: var(--background-depth-one-color);
        box-shadow: var(--box-shadow);
        opacity: 0;
        padding: 0.5rem;
        border-radius: 0.5rem;
        box-sizing: border-box;
        overflow-x: hidden;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        transform: scale(1.125);

        .rd-input-option {
          cursor: pointer;
          position: relative;
          width: 100%;
          height: 2rem;
          background: rgba(0, 0, 0, 0);
          padding: 0 0.75rem;
          border-radius: 0.25rem;
          box-sizing: border-box;
          display: flex;
          flex-shrink: 0;
          align-items: center;
          transition: background-color 0.25s;

          &.rd-input-option-selected {
            background: rgba(0, 0, 0, 0.05);
          }
        }

        &.rd-input-options-opened {
          pointer-events: all;
        }
      }
      .rd-input-chevron-container {
        z-index: 2;
        pointer-events: none;
        position: absolute;
        right: 0;
        width: 2rem;
        height: 2rem;
        padding: 0 0.5rem;
        border-top-left-radius: 0.5rem;
        border-bottom-left-radius: 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.25s rotate;
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
    &.rd-input-disabled {
      pointer-events: none;
      filter: grayscale(0.75);
      opacity: 0.5;
    }
    &.rd-input-error-active {
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
  }
</style>
