<template>
  <button
    class="rd-input-component"
    ref="rdInputComponent"
    @mousedown="mouseDownHandler"
    :class="disabled ? 'rd-input-component-disabled' : ''"
    :disabled="disabled"
  >
    <div class="rd-input-icon-container">
      <rd-svg :name="props.icon" />
    </div>
    <div class="rd-input-overlay"></div>
  </button>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    icon: string;
    disabled?: boolean;
  }>();
  const emits = defineEmits(["clicked"]);

  const rdInputComponent = ref<HTMLButtonElement | null>(null);

  const buttonAnimating = ref<boolean>(false);
  const buttonClicking = ref<boolean>(false);
  const buttonPressed = ref<boolean>(false);

  const animate = {
    click(rdInputComponent: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdOverlay: HTMLElement | null =
        rdInputComponent.querySelector(".rd-input-overlay");

      tl.to(rdInputComponent, {
        scale: 0.95,
        duration: 0.25,
        ease: "power2.out",
      }).to(
        rdOverlay,
        {
          opacity: 1,
          duration: 0.25,
          ease: "power2.inOut",
        },
        "<0"
      );
    },
    release(rdInputComponent: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdOverlay: HTMLElement | null =
        rdInputComponent.querySelector(".rd-input-overlay");

      tl.to(rdInputComponent, {
        scale: 1,
        duration: 0.25,
        ease: "back.out(2)",
      }).to(
        rdOverlay,
        {
          opacity: 0,
          duration: 0.25,
          ease: "power2.out",
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
      animate.click(rdInputComponent.value, () => {
        buttonClicking.value = false;
        if (!buttonPressed.value) {
          mouseUpHandler();
        }
      });
    }
    window.addEventListener("mouseup", mouseUpHandler);
  }
  function mouseUpHandler(): void {
    window.removeEventListener("mouseup", mouseUpHandler);
    buttonPressed.value = false;
    if (!buttonClicking.value && rdInputComponent.value) {
      emits("clicked");
      animate.release(rdInputComponent.value, () => {
        buttonAnimating.value = false;
      });
    }
  }

  onBeforeUnmount(() => {
    window.removeEventListener("mouseup", mouseUpHandler);
  });
</script>

<style lang="scss" scoped>
  button.rd-input-component {
    cursor: pointer;
    position: relative;
    width: 2rem;
    height: 1rem;
    display: flex;
    border-radius: 0.5rem;
    margin: 0;
    background: var(--background-depth-one-color);
    border: none;
    padding: 0;
    overflow: hidden;
    display: flex;
    align-items: center;
    * {
      pointer-events: none;
    }
    .rd-input-icon-container {
      position: relative;
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
    }
    .rd-input-icon-container {
      padding: 0 0.625rem;
      box-sizing: border-box;
    }
    .rd-input-overlay {
      pointer-events: none;
      position: absolute;
      top: 0;
      left: 0;
      border-radius: 0.5rem;
      width: 100%;
      height: 100%;
      background: rgba(0, 0, 0, 0.1);
      opacity: 0;
    }
    &.rd-input-component-disabled {
      pointer-events: none;
      filter: grayscale(0.75);
      opacity: 0.5;
    }
  }
</style>
