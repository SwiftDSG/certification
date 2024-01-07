<template>
  <button
    class="rd-input-component"
    ref="rdInputComponent"
    :class="`${props.disabled ? 'rd-input-component-disabled' : ''} ${
      props.loading || buttonAnimating ? 'rd-input-component-animating' : ''
    } rd-input-component-${props.type ? props.type : 'primary'}`"
    :disabled="props.disabled"
    @mousedown="mouseDownHandler"
  >
    <div class="rd-input-label-container">
      <label class="rd-input-label rd-body-text rd-input-label-decoy">
        {{ props.label }}
      </label>
      <label class="rd-input-label rd-headline-6 rd-input-label-main">
        <span
          class="rd-letter"
          :class="letter === ' ' ? 'rd-letter-space' : ''"
          v-for="(letter, i) in props.label"
          :key="i"
        >
          {{ letter }}
        </span>
      </label>
      <label class="rd-input-label rd-body-text rd-input-label-overlay">
        <span
          class="rd-letter"
          :class="letter === ' ' ? 'rd-letter-space' : ''"
          v-for="(letter, i) in props.label"
          :key="i"
        >
          {{ letter }}
        </span>
      </label>
    </div>
    <div class="rd-input-progress-bar">
      <div class="rd-input-progress-bar-outer">
        <div class="rd-input-progress-bar-inner"></div>
      </div>
    </div>
    <div class="rd-input-overlay"></div>
  </button>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    label: string;
    type?: "primary" | "secondary" | "error" | "background";
    disabled?: boolean;
    loading?: boolean;
  }>();
  const emits = defineEmits(["clicked"]);

  const rdInputComponent = ref<HTMLButtonElement | null>(null);

  const buttonAnimating = ref<boolean>(false);
  const buttonClicking = ref<boolean>(false);
  const buttonPressed = ref<boolean>(false);
  const buttonHovered = ref<boolean>(false);
  const buttonLoadingAnim = ref<GSAPTimeline | null>(null);

  const animate = {
    hover(rdInputComponent: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          gsap.to(rdLetter, {
            y: 0,
            opacity: 1,
            duration: 0,
          });
          gsap.to(rdOverlayLetter, {
            y: "100%",
            opacity: 0,
            duration: 0,
          });
          if (cb) cb();
        },
      });

      const rdLetter: HTMLElement[] = gsap.utils.toArray(
        rdInputComponent.querySelectorAll(
          "label.rd-input-label-main .rd-letter"
        )
      );
      const rdOverlayLetter: HTMLElement[] = gsap.utils.toArray(
        rdInputComponent.querySelectorAll(
          "label.rd-input-label-overlay .rd-letter"
        )
      );

      tl.to(rdLetter, {
        y: "-100%",
        opacity: 0,
        duration: 0.25,
        ease: "power2.in",
        stagger: 0.05,
      }).to(
        rdOverlayLetter,
        {
          y: 0,
          opacity: 1,
          duration: 0.25,
          ease: "powr2.out",
          stagger: 0.05,
        },
        "<0.125"
      );
    },
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
    loading(rdInputComponent: HTMLElement, cb?: () => void): GSAPTimeline {
      const tl: GSAPTimeline = gsap.timeline({});

      const rdLetter: HTMLElement[] = gsap.utils.toArray(
        rdInputComponent.querySelectorAll(
          "label.rd-input-label-main .rd-letter"
        )
      );
      const rdProgressBar: HTMLElement | null = rdInputComponent.querySelector(
        ".rd-input-progress-bar"
      );

      tl.to(rdLetter, {
        y: "-100%",
        opacity: 0,
        duration: 0.25,
        ease: "power2.in",
        stagger: 0.05,
      }).to(rdProgressBar, {
        opacity: 1,
        duration: 0.25,
        ease: "powr2.out",
      });

      return tl;
    },
  };

  function mouseDownHandler(e: MouseEvent): MouseEvent {
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
    return e;
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

  watch(
    () => props.loading,
    (val) => {
      setTimeout(() => {
        if (!buttonLoadingAnim.value && rdInputComponent.value) {
          buttonLoadingAnim.value = animate.loading(rdInputComponent.value);
        }
        if (buttonLoadingAnim.value) {
          if (val) buttonLoadingAnim.value.play();
          else buttonLoadingAnim.value.reverse();
        }
      }, 100);
    }
  );

  onBeforeUnmount(() => {
    window.removeEventListener("mouseup", mouseUpHandler);
  });
</script>

<style lang="scss" scoped>
  button.rd-input-component {
    cursor: pointer;
    position: relative;
    height: 1.5rem;
    display: flex;
    background: var(--primary-color);
    border-radius: 0.75rem;
    border: 1px solid var(--primary-color);
    padding: 0;
    margin: 0;
    overflow: hidden;
    display: flex;
    align-items: center;
    * {
      pointer-events: none;
    }
    .rd-input-label-container {
      position: relative;
      width: 100%;
      height: 100%;
      padding: 0 0.5rem;
      box-sizing: border-box;
      display: flex;
      justify-content: center;
      align-items: center;
      label.rd-input-label {
        position: absolute;
        width: 100%;
        height: 100%;
        color: #fff;
        display: flex;
        justify-content: center;
        align-items: center;
        span.rd-letter {
          position: relative;
          display: flex;
          color: inherit;
          line-height: 1;
          &.rd-letter-space {
            width: 0.125rem;
            height: 100%;
          }
        }
        &.rd-input-label-overlay {
          span.rd-letter {
            transform: translateY(100%);
            opacity: 0;
          }
        }
        &.rd-input-label-decoy {
          position: relative;
          opacity: 0;
          visibility: hidden;
        }
      }
    }
    .rd-input-progress-bar {
      position: absolute;
      top: calc(50% - 0.375rem);
      left: calc(50% - 0.375rem);
      width: 0.75rem;
      height: 0.75rem;
      opacity: 0;
      animation: rd-rotate 500ms linear infinite;
      .rd-input-progress-bar-outer {
        position: absolute;
        top: 0;
        left: 0;
        width: 0.375rem;
        height: 0.75rem;
        overflow: hidden;
        .rd-input-progress-bar-inner {
          position: absolute;
          top: 0;
          left: 0;
          width: 0.75rem;
          height: 0.75rem;
          border: 2px solid #fff;
          border-radius: 50%;
          border-left-color: transparent;
          border-bottom-color: transparent;
          box-sizing: border-box;
          animation: rd-circular-rotate 1000ms cubic-bezier(0.4, 0, 0.22, 1)
            infinite;
        }
      }
    }
    .rd-input-overlay {
      pointer-events: none;
      position: absolute;
      top: 0;
      left: 0;
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
    &.rd-input-component-animating {
      pointer-events: none !important;
    }
    &.rd-input-component-icon {
      .rd-input-label-container {
        width: calc(100% - 2rem);
      }
      .rd-input-icon-container {
        position: relative;
        width: 2rem;
        height: 2rem;
        background: rgba(0, 0, 0, 0.05);
        padding: 0 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
      }
    }
    &.rd-input-component-secondary {
      background: transparent;
      border: var(--border);
      box-sizing: border-box;
      .rd-input-label-container {
        label.rd-input-label {
          color: var(--font-primary-color);
        }
      }
    }
    &.rd-input-component-background {
      background: var(--background-depth-one-color);
      .rd-input-label-container {
        label.rd-input-label {
          color: var(--font-primary-color);
        }
      }
    }
    &.rd-input-component-error {
      background: transparent;
      border: 1px solid var(--error-color);
      box-sizing: border-box;
      .rd-input-label-container {
        label.rd-input-label {
          color: var(--error-color);
        }
      }
    }
  }
</style>
