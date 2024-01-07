<template>
  <div ref="rdInputComponent" class="rd-input-component" :class="`${input.error ? 'rd-input-error' : ''} ${input.disabled || (input.strict && !input.options?.length)
    ? 'rd-input-disabled'
    : ''
    }`" @focusout="focusHandler">
    <label v-if="input.label" :for="id" class="rd-input-label rd-headline-6">{{
      input.label
    }}</label>
    <div class="rd-input-container">
      <div v-if="input.icon" class="rd-input-icon-container">
        <rd-svg :name="input.icon" />
      </div>
      <input class="rd-input rd-body-text" :placeholder="input.placeholder" ref="rdInput" :name="input.name" type="text"
        :id="id" :disabled="input.disabled || (input.strict && !input.options?.length)" @focus="dropDownHandler('open')"
        @input="updateModel" :readonly="view === 'mobile'" />
      <div class="rd-input-border"></div>
      <div class="rd-input-chevron-container" :style="dropDownState === 'opened' ? 'rotate: 180deg' : ''">
        <rd-svg name="chevron-down" />
      </div>
    </div>
    <span v-if="typeof input.error === 'string'" class="rd-input-error rd-caption-text">
      <span class="rd-text-wrapper">
        <span class="rd-text-container rd-text-container-up">
          <span class="rd-text">{{ inputError }}</span>
        </span>
      </span>
    </span>
    <rd-popover class="rd-input-options" :state="dropDownState" :limit="9" @exit="removeFocus">
      <span v-if="!inputOptions.length" class="rd-input-empty rd-caption-text">Option unavailable</span>
      <div class="rd-input-option" v-for="(option, i) in inputOptions" :key="option.value" :data-index="i"
        ref="rdInputOption" @mousedown="selectOption(i)" @mouseenter="dropDownIndex = i" :class="`${input.model.value === option.value ? 'rd-input-option-selected' : ''} ${dropDownIndex === i ? 'rd-input-option-hovered' : ''}`
          " :aria-selected="input.model.value === option.value">
        <span class="rd-input-option-text rd-body-text">
          {{ option.name }}
        </span>
      </div>
    </rd-popover>
  </div>
</template>

<script lang="ts" setup>
import { InputSelectOption } from "~~/types/general.js";

const props = defineProps<{
  input: InputSelectOption;
}>();
const emits = defineEmits(["focusout"]);
const { view } = useMain();

const rdInputComponent = ref<HTMLDivElement | null>(null);
const rdInput = ref<HTMLInputElement | null>(null);
const rdInputOption = ref<HTMLDivElement[] | null>(null);

const inputError = ref<string | undefined>(props.input.error);
const inputModel = ref<InputSelectOption["model"]>({
  name: "",
  value: "",
});
const inputOptions = ref<InputSelectOption["options"]>(props.input.options);

const dropDownState = ref<"opened" | "closed">("closed");
const dropDownIndex = ref<number>(-1);

const id = computed<string>(() => {
  const characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let str = "";
  for (let i = 0; i < 10; i++) {
    str += characters[Math.round(Math.random() * (characters.length - 1))];
  }

  return `rd-input-${str}`;
});

function focusHandler(e: FocusEvent): void {
  if (
    !e.relatedTarget ||
    (e.relatedTarget instanceof HTMLElement &&
      !rdInputComponent.value?.contains(e.relatedTarget))
  ) {
    dropDownHandler("close", true);
    emits("focusout", props.input);
  } else if (e.relatedTarget instanceof HTMLElement) {
    const index = e.relatedTarget.dataset.index;
    if (index) dropDownIndex.value = parseInt(index);
  }
}
function keyHandler(e: KeyboardEvent): void {
  if (e.key === "ArrowUp") {
    e.preventDefault();
    selectIndex(
      dropDownIndex.value <= 0
        ? inputOptions.value.length - 1
        : dropDownIndex.value - 1
    );
  } else if (e.key === "ArrowDown") {
    e.preventDefault();
    selectIndex(
      dropDownIndex.value >= inputOptions.value.length - 1
        ? 0
        : dropDownIndex.value + 1
    );
  } else if (e.key === "Enter") {
    selectOption(dropDownIndex.value);
  } else if (e.key === "Escape") {
    removeFocus();
  } else if (e.key !== "Tab" && rdInput.value) {
    rdInput.value.focus();
  }
}
function removeFocus(): void {
  dropDownState.value = "closed";
  if (document.activeElement instanceof HTMLElement) {
    document.activeElement.blur();
  }
}
function dropDownHandler(state: "open" | "close", validate?: boolean): void {
  if (state === "open") {
    dropDownState.value = "opened";
    dropDownIndex.value =
      inputOptions.value?.findIndex(
        (a) => a.value === props.input.model.value
      ) || 0;
    if (view.value === "desktop")
      window.addEventListener("keydown", keyHandler);
  } else if (rdInput.value) {
    window.removeEventListener("keydown", keyHandler);

    dropDownIndex.value = -1;
    dropDownState.value = "closed";

    if (validate) {
      if (inputModel.value.name) {
        if (
          !props.input.options.find((a) => a.value === inputModel.value.value)
        ) {
          const selection = props.input.options.find((a) =>
            a.name
              .replace(/[^A-Za-z0-9]/g, "")
              .toLowerCase()
              .includes(
                inputModel.value.name
                  .replace(/[^A-Za-z0-9]/g, "")
                  .toLowerCase()
              )
          );
          if (selection) {
            rdInput.value.value = selection.name;
            inputModel.value = {
              name: selection.name,
              value: selection.value,
            };
            props.input.model = selection;
          } else if (props.input.strict) {
            rdInput.value.value = "";
            inputModel.value = {
              name: "",
              value: "",
            };
            props.input.model = {
              name: "",
              value: "",
            };
            setTimeout(() => {
              filterOptions("");
            }, 250);
          }
        }
      } else {
        rdInput.value.value = "";
        inputModel.value = {
          name: "",
          value: "",
        };
        props.input.model = {
          name: "",
          value: "",
        };
        setTimeout(() => {
          filterOptions("");
        }, 250);
      }
    }
  }
}
function updateModel(e: Event): void {
  if (e.target instanceof HTMLInputElement) {
    inputModel.value.name = e.target.value;
    if (!props.input.strict) {
      props.input.model = {
        name: inputModel.value.name,
        value: inputModel.value.name,
      };
    }
    filterOptions(e.target.value);
    if (dropDownState.value !== "opened") dropDownHandler("open");
  }
}
function filterOptions(str: string): void {
  if (str.length) {
    inputOptions.value = props.input.options
      .filter((a) =>
        a.name
          .replace(/[^A-Za-z0-9]/g, "")
          .toLowerCase()
          .includes(str.replace(/[^A-Za-z0-9]/g, "").toLowerCase())
      )
      .sort((a, b) => (a.name.toLowerCase() > b.name.toLowerCase() ? 1 : -1));
  } else {
    inputOptions.value = props.input.options.sort((a, b) =>
      a.name.toLowerCase() > b.name.toLowerCase() ? 1 : -1
    );
  }
  dropDownIndex.value = 0;
}
function selectIndex(index: number): void {
  const rdInputOption = rdInputComponent.value?.querySelector(
    `[data-index="${index}"]`
  );
  if (rdInputOption instanceof HTMLElement) {
    dropDownIndex.value = index;
    rdInputOption.focus();
  }
}
function selectOption(index: number): void {
  if (inputOptions.value) {
    const selection = inputOptions.value[index];
    if (selection && rdInput.value) {
      inputModel.value = {
        name: selection.name,
        value: selection.value,
      };
      props.input.model = inputModel.value;
      rdInput.value.value = inputModel.value.name;
      rdInput.value.focus();
      setTimeout(() => {
        filterOptions(inputModel.value.name)
      }, 250);
    }
    dropDownHandler("close");
  }
}

watch(
  () => props.input.error,
  (val) => {
    if (val) inputError.value = val;
  }
);
watch(
  () => props.input.options,
  (val) => {
    inputOptions.value = (
      JSON.parse(JSON.stringify(val)) as InputSelectOption["options"]
    ).sort((a, b) => (a.name.toLowerCase() > b.name.toLowerCase() ? 1 : -1));
  },
  {
    deep: true,
  }
);
watch(
  () => props.input.model,
  (val) => {
    if (val.name) {
      const selection = props.input.options.find(
        (a) => a.name === val.name || a.value === val.value
      );
      if (selection && rdInput.value) {
        rdInput.value.value = selection.name;
        inputModel.value = {
          name: selection.name,
          value: selection.value,
        };
      }
    } else if (rdInput.value) {
      dropDownIndex.value = -1;
      inputModel.value = {
        name: "",
        value: "",
      };
      rdInput.value.value = "";
    }
  },
  { deep: true }
);

onBeforeUnmount(() => {
  window.removeEventListener("keydown", keyHandler);
});
onMounted(() => {
  if (props.input.model.name && rdInput.value) {
    inputModel.value.name = props.input.model.name;
    rdInput.value.value = inputModel.value.name;
  }
  if (props.input.model.value) {
    inputModel.value.value = props.input.model.value;
  }
});
</script>

<style lang="scss" scoped>
.rd-input-component {
  cursor: pointer;
  position: relative;
  width: 100%;
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
      pointer-events: none;
      z-index: 2;
      position: absolute;
      width: 2rem;
      height: 2rem;
      padding: 0 0.5rem;
      border-top-left-radius: 0.5rem;
      border-bottom-left-radius: 0.5rem;
      box-sizing: border-box;
      display: flex;
      justify-content: center;
      align-items: center;

      ~input.rd-input {
        padding: 0 2rem;
      }
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

      &:focus+.rd-input-border {
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

    .rd-input-chevron-container {
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

    * {
      pointer-events: none;
    }

    span.rd-text-wrapper {
      span.rd-text-container {
        transition: 0.25s transform;

        span.rd-text {
          transition: 0.25s transform;
        }
      }
    }

    &~.rd-input-options {
      top: calc(100% + 2px - 1rem);
    }
  }

  .rd-input-options {
    top: calc(100% + 2px);

    span.rd-input-empty {
      pointer-events: none;
      position: relative;
      width: 100%;
      height: 2rem;
      display: flex;
      justify-content: center;
      align-items: center;
    }

    .rd-input-option {
      cursor: pointer;
      position: relative;
      width: 100%;
      height: 2rem;
      padding: 0 0.75rem;
      border-radius: 0.25rem;
      box-sizing: border-box;
      display: flex;
      flex-shrink: 0;
      align-items: center;

      * {
        pointer-events: none;
      }

      span.rd-input-option-text {
        position: relative;
        width: 100%;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      &::before {
        content: "";
        position: absolute;
        left: 0;
        width: 100%;
        height: 100%;
        border-radius: 0.25rem;
        background: #000;
        opacity: 0;
        transition: background-color 0.25s, opacity 0.25s;
      }

      &.rd-input-option-hovered {
        outline: none;

        &::before {
          background: #000;
          opacity: 0.05;
        }
      }

      &.rd-input-option-selected {
        span.rd-input-option-text {
          color: var(--primary-color);
        }

        &::before {
          background: var(--primary-color);
          opacity: 0.05;
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

  &:focus-within {
    .rd-input-container {
      input.rd-input {
        outline: none;
        background: var(--background-depth-two-color);

        &+.rd-input-border {
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
