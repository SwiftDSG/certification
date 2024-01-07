<template>
  <div
    class="rd-input-component"
    ref="rdInputComponent"
    :class="input.disabled ? 'rd-input-component-disabled' : ''"
  >
    <input
      v-for="(option, i) in input.options"
      :key="i"
      type="radio"
      class="rd-input"
      name="rd-input-radio"
      ref="rdInput"
      :id="`${id}-${i}`"
      :disabled="input.disabled"
      :data-index="i"
      @change="updateModel"
    />
    <label
      v-for="(option, i) in input.options"
      :key="i"
      :for="`${id}-${i}`"
      :class="props.input.model === option ? 'rd-input-label-active' : ''"
      class="rd-input-label"
      ref="rdInputLabel"
    >
      <span class="rd-input-label-option rd-body-text">{{ option }}</span>
    </label>
    <div ref="rdInputBorder" class="rd-input-border"></div>
  </div>
</template>

<script lang="ts" setup>
  import { InputSwitchOption } from "~~/types/general";
  import gsap from "gsap";

  type Size = {
    left: number;
    width: number;
  };

  const props = defineProps<{
    input: InputSwitchOption;
  }>();
  const { rem } = useMain();

  const rdInputLabel = ref<HTMLLabelElement[] | null>(null);
  const rdInputComponent = ref<HTMLDivElement | null>(null);
  const rdInputBorder = ref<HTMLDivElement | null>(null);

  const windowLeft = ref<number>(0);
  const sizes = ref<Size[]>([]);

  const id = computed<string>(() => {
    const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let combination = "";
    for (let i = 0; i < 6; i++) {
      combination += chars[Math.round(Math.random() * 35)];
    }
    return `rd-input-${combination}`;
  });

  function updateModel(e: Event): Event {
    if (e.target instanceof HTMLInputElement) {
      const index: number = parseInt(e.target.dataset.index || "-1");
      if (index > -1) {
        props.input.model = props.input.options[index];
        const { width, left }: Size = sizes.value[index];
        gsap.to(rdInputBorder.value, {
          x: left - windowLeft.value - 0.75 * rem.value,
          width,
          duration: 0.25,
          ease: "power2.out",
        });
      }
    }
    return e;
  }

  watch(
    () => props.input.model,
    (val) => {
      const index = props.input.options.findIndex((a) => a === val);
      if (index > -1) {
        const { width, left }: Size = sizes.value[index];
        gsap.to(rdInputBorder.value, {
          x: left - windowLeft.value - 0.75 * rem.value,
          width,
          duration: 0.25,
          ease: "power2.out",
        });
      }
    }
  );

  onMounted(() => {
    if (rdInputComponent.value && rdInputLabel.value) {
      windowLeft.value = rdInputComponent.value.getBoundingClientRect().left;
      for (const rdElement of rdInputLabel.value) {
        const { width, left }: DOMRect = rdElement.getBoundingClientRect();
        sizes.value.push({ width, left });
      }

      const rdInput = rdInputComponent.value.querySelector("input");
      if (rdInput) rdInput.click();
    }
  });
</script>

<style lang="scss" scoped>
  .rd-input-component {
    position: relative;
    width: 100%;
    padding: 0 0.75rem;
    box-sizing: border-box;
    display: flex;
    gap: 0.75rem;
    align-items: center;
    overflow-x: auto;
    input.rd-input {
      pointer-events: none;
      position: fixed;
      top: -1rem;
      left: -1rem;
      opacity: 0;
      &:checked + label.rd-input-label {
        .rd-input-label-slider {
          transform: translateX(100%);
          span.rd-input-label-slider-option {
            transform: translateX(-100%);
          }
        }
      }
    }
    label.rd-input-label {
      cursor: pointer;
      position: relative;
      padding-bottom: 0.5rem;
      box-sizing: border-box;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      * {
        pointer-events: none;
      }
      span.rd-input-label-option {
        position: relative;
        width: 100%;
        white-space: nowrap;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.2s color;
      }
      &.rd-input-label-active {
        span.rd-input-label-option {
          color: var(--primary-color);
        }
      }
    }
    .rd-input-border {
      position: absolute;
      bottom: 0;
      height: 2px;
      background: var(--primary-color);
    }
    &.rd-input-component-disabled {
      pointer-events: none;
      opacity: 0.5;
    }
    &::-webkit-scrollbar {
      display: none;
    }
  }
</style>
