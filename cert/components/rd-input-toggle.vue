<template>
  <div class="rd-input-component" ref="rdInputComponent">
    <label
      class="rd-input-label"
      @mousedown="mouseDownHandler"
      :class="`${buttonAnimating ? 'rd-input-label-animating ' : ''}${
        input.model ? 'rd-input-label-active ' : ''
      }`"
    >
      <div class="rd-input-label-slider"></div>
    </label>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { InputToggleOption } from "~~/types/general";

  const props = defineProps<{
    input: InputToggleOption;
  }>();

  const emits = defineEmits(["clicked"]);
  const rdInputComponent = ref<HTMLButtonElement | null>(null);

  const buttonAnimating = ref<boolean>(false);
  const buttonClicking = ref<boolean>(false);
  const buttonPressed = ref<boolean>(false);

  const animate = {
    click(
      toggled: boolean,
      rdInputComponent: HTMLElement,
      cb?: () => void
    ): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdInputSlider: HTMLElement | null = rdInputComponent.querySelector(
        ".rd-input-label-slider"
      );

      tl.to(rdInputSlider, {
        width: "1rem",
        x: toggled ? "-=0.25rem" : 0,
        duration: 0.25,
        ease: "power2.inOut",
      });
    },
    release(
      toggled: boolean,
      rdInputComponent: HTMLElement,
      cb?: () => void
    ): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdInputLabel: HTMLElement | null =
        rdInputComponent.querySelector(".rd-input-label");
      const rdInputSlider: HTMLElement | null = rdInputComponent.querySelector(
        ".rd-input-label-slider"
      );

      tl.to(rdInputLabel, {
        backgroundColor: toggled
          ? "var(--background-depth-three-color)"
          : "var(--primary-color)",
      }).to(
        rdInputSlider,
        {
          width: "0.75rem",
          x: toggled ? 0 : "0.5rem",
          duration: 0.25,
          ease: "power2.inOut",
        },
        "<0"
      );
    },
  };

  function mouseDownHandler(): void {
    buttonAnimating.value = true;
    buttonClicking.value = true;
    buttonPressed.value = true;
    if (rdInputComponent.value) {
      animate.click(props.input.model, rdInputComponent.value, () => {
        emits("clicked");
        buttonClicking.value = false;
        if (!buttonPressed.value) {
          mouseUpHandler();
        }
      });
      window.addEventListener("mouseup", mouseUpHandler);
    }
  }
  function mouseUpHandler(): void {
    window.removeEventListener("mouseup", mouseUpHandler);
    buttonPressed.value = false;
    if (!buttonClicking.value && rdInputComponent.value) {
      animate.release(props.input.model, rdInputComponent.value, () => {
        props.input.model = !props.input.model;
        setTimeout(() => {
          buttonAnimating.value = false;
        }, 100);
      });
    }
  }

  watch(
    () => props.input.model,
    (val) => {
      if (!buttonAnimating.value && rdInputComponent.value) {
        buttonAnimating.value = true;
        animate.click(!val, rdInputComponent.value, () => {
          if (rdInputComponent.value) {
            animate.release(!val, rdInputComponent.value, () => {
              buttonAnimating.value = false;
            });
          }
        });
      }
    }
  );

  onMounted(() => {
    if (props.input.model && rdInputComponent.value) {
      animate.click(false, rdInputComponent.value, () => {
        if (rdInputComponent.value) {
          animate.release(false, rdInputComponent.value, () => {
            buttonAnimating.value = false;
          });
        }
      });
    }
  });
</script>

<style lang="scss" scoped>
  .rd-input-component {
    position: relative;
    width: 1.5rem;
    height: 1rem;
    display: flex;
    align-items: center;
    label.rd-input-label {
      cursor: pointer;
      position: relative;
      width: 100%;
      height: 100%;
      background: var(--background-depth-three-color);
      border-radius: 0.5rem;
      padding: 0.125rem;
      box-sizing: border-box;
      transition: 0.25s ease-in-out background-color;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      * {
        pointer-events: none;
      }
      .rd-input-label-slider {
        position: absolute;
        top: 0.125rem;
        left: 0.125rem;
        width: 0.75rem;
        height: 0.75rem;
        border-radius: 0.375rem;
        box-sizing: border-box;
        background: var(--background-depth-one-color);
        overflow: hidden;
        display: flex;
        align-items: center;
      }
      &.rd-input-label-animating {
        pointer-events: none;
      }
      &.rd-input-label-active {
        background: var(--primary-color);
      }
    }
  }
</style>
