<template>
  <label
    class="rd-input-component"
    :class="props.input.disabled ? 'rd-input-component-disabled' : ''"
  >
    <input
      type="file"
      class="rd-input"
      :accept="input.type === 'image' ? 'image/*' : '*/*'"
      @change="changeHandler"
    />
    <div
      v-if="!file"
      ref="rdInputLabel"
      class="rd-input-label"
      :class="init ? 'rd-input-label-init' : ''"
    >
      <span class="rd-input-placeholder rd-caption-text">
        Click
        <span class="rd-input-placeholder-highlight rd-headline-6">here</span>
        {{ `to upload ${input?.type === "image" ? "image" : "file"}` }}
      </span>
    </div>
    <div v-else ref="rdInputDetails" class="rd-input-details-container">
      <div class="rd-input-icon-container">
        <rd-svg :name="iconHandler(file.name)" color="primary" />
      </div>
      <div class="rd-input-details">
        <span class="rd-input-value rd-headline-5">{{ file.name }}</span>
        <span class="rd-input-placeholder rd-caption-text">{{
          sizeHandler(file.size)
        }}</span>
      </div>
      <div class="rd-input-remove-button" @click.prevent="removeFile">
        <rd-svg :name="'close'" :color="'secondary'" />
      </div>
    </div>
    <div ref="rdInputLoading" class="rd-input-loading">
      <div class="rd-input-icon-progress-bar">
        <div class="rd-input-icon-progress-bar-outer">
          <div class="rd-input-icon-progress-bar-inner"></div>
        </div>
      </div>
    </div>
  </label>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  import { InputFileOption } from "~~/types/general.js";

  interface AnyFile {
    name: string;
    type: string;
    size: number;
    file: File;
  }

  const props = defineProps<{
    input: InputFileOption;
  }>();

  const rdInputLabel = ref<HTMLDivElement | null>(null);
  const rdInputDetails = ref<HTMLDivElement | null>(null);
  const rdInputLoading = ref<HTMLDivElement | null>(null);

  const file = ref<AnyFile | null>(null);
  const init = ref<boolean>(true);

  const animate = {
    initLabel(rdInputLabel: HTMLElement): void {
      const tl: GSAPTimeline = gsap.timeline();

      const rdInputPlaceholder: HTMLElement | null = rdInputLabel.querySelector(
        "span.rd-input-placeholder"
      );

      tl.to(rdInputPlaceholder, {
        scale: 0.875,
        opacity: 0,
        duration: 0,
      }).to(rdInputPlaceholder, {
        scale: 1,
        opacity: 1,
        duration: 0.25,
        ease: "power2.out",
      });
    },
    exitLabel(rdInputLabel: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdInputPlaceholder: HTMLElement | null = rdInputLabel.querySelector(
        "span.rd-input-placeholder"
      );

      tl.to(rdInputPlaceholder, {
        scale: 1.125,
        opacity: 0,
        duration: 0.25,
        ease: "power2.out",
      }).to(rdInputPlaceholder, {
        scale: 0.875,
        opacity: 0,
        duration: 0,
      });
    },
    initDetails(rdInputDetails: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      tl.to(rdInputDetails, {
        scale: 0.875,
        opacity: 0,
        duration: 0,
      }).to(rdInputDetails, {
        scale: 1,
        opacity: 1,
        duration: 0.25,
        ease: "power2.out",
      });
    },
    exitDetails(rdInputDetails: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      tl.to(rdInputDetails, {
        scale: 1.125,
        opacity: 0,
        duration: 0.25,
        ease: "power2.out",
      }).to(rdInputDetails, {
        scale: 0.875,
        opacity: 0,
        duration: 0,
      });
    },
    initLoading(rdInputLoading: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdInputProgressBar: HTMLElement | null =
        rdInputLoading.querySelector(".rd-input-icon-progress-bar");

      tl.to(rdInputProgressBar, {
        scale: 0.875,
        opacity: 0,
        duration: 0,
      }).to(rdInputProgressBar, {
        scale: 1,
        opacity: 1,
        duration: 0.25,
        ease: "power2.out",
      });
    },
    exitLoading(rdInputLoading: HTMLElement, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const rdInputProgressBar: HTMLElement | null =
        rdInputLoading.querySelector(".rd-input-icon-progress-bar");

      tl.to(rdInputProgressBar, {
        scale: 1.125,
        opacity: 0,
        duration: 0.25,
        ease: "power2.out",
      }).to(rdInputProgressBar, {
        scale: 0.875,
        opacity: 0,
        duration: 0,
      });
    },
  };

  function iconHandler(name: string): string {
    const str: string = name.split(".").reverse()[0];
    if (!str) return "file";
    if (str === "svg") return "image";
    if (str === "png") return "image";
    if (str === "jpg") return "image";
    if (str === "jpeg") return "image";
    if (str === "doc" || str === "docs") return "word";
    if (str === "xlx" || str === "xlxs" || str === "csv") return "excel";
    if (str === "pdf") return "pdf";
    return "file";
  }
  function sizeHandler(size: number): string {
    if (size > 1000000) return `${(size / 1000000).toFixed(1)} mb`;
    if (size > 1000) return `${(size / 1000).toFixed(1)} kb`;
    return `${size} b`;
  }

  function removeFile(): void {
    if (rdInputDetails.value) {
      animate.exitDetails(rdInputDetails.value, () => {
        file.value = null;
        setTimeout(() => {
          if (rdInputLabel.value) animate.initLabel(rdInputLabel.value);
        }, 100);
      });
    }
  }

  function changeHandler(e: Event): void {
    e.preventDefault();
    if (e.target instanceof HTMLInputElement) {
      const files = e.target.files;
      if (files) fileHandler(files);
    }
  }
  function fileHandler(files: FileList): void {
    if (rdInputLabel.value) {
      animate.exitLabel(rdInputLabel.value, () => {
        if (rdInputLoading.value) {
          init.value = false;
          file.value = {
            name: files[0].name,
            type: files[0].type,
            size: files[0].size,
            file: files[0],
          };
          props.input.file = files[0];
          animate.initLoading(rdInputLoading.value, () => {
            setTimeout(() => {
              if (rdInputLoading.value) {
                animate.exitLoading(rdInputLoading.value, () => {
                  if (rdInputDetails.value) {
                    animate.initDetails(rdInputDetails.value);
                  }
                });
              }
            }, 500);
          });
        }
      });
    }
  }
</script>

<style lang="scss" scoped>
  label.rd-input-component {
    position: relative;
    width: 100%;
    height: 3.5rem;
    padding: 0.75rem;
    border-radius: 0.5rem;
    box-sizing: border-box;
    overflow: hidden;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    input.rd-input {
      z-index: -1;
      position: absolute;
      visibility: hidden;
    }
    .rd-input-label {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      span.rd-input-placeholder {
        position: relative;
        opacity: 0;
        transform: scale(0.875);
        .rd-input-placeholder-highlight {
          cursor: pointer;
          position: relative;
          color: var(--primary-color);
        }
      }
      &.rd-input-label-init {
        span.rd-input-placeholder {
          opacity: 1;
          transform: scale(1);
        }
      }
    }
    .rd-input-details-container {
      position: relative;
      width: 100%;
      height: 2rem;
      opacity: 0;
      display: flex;
      align-items: center;
      transform: scale(0.875);
      .rd-input-icon-container {
        position: relative;
        width: 2rem;
        height: 2rem;
        background: var(--background-depth-two-color);
        border-radius: 0.5rem;
        padding: 0 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
      }

      .rd-input-details {
        position: absolute;
        top: 0;
        left: 2rem;
        width: calc(100% - 2rem);
        height: 100%;
        padding-left: 0.5rem;
        box-sizing: border-box;
        border-radius: 0.5rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        span.rd-input-value {
          position: relative;
          width: calc(100% - 2rem);
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }
        span.rd-input-placeholder {
          position: relative;
          opacity: 0.5;
          margin-top: 0.125rem;
        }
      }
      .rd-input-remove-button {
        cursor: pointer;
        position: absolute;
        top: 0;
        right: -0.25rem;
        width: 2rem;
        height: 2rem;
        padding: 0 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
      }
    }
    .rd-input-loading {
      pointer-events: none;
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      .rd-input-icon-progress-bar {
        position: absolute;
        top: calc(50% - 0.375rem);
        left: calc(50% - 0.375rem);
        width: 0.75rem;
        height: 0.75rem;
        opacity: 0;
        animation: rd-rotate 500ms linear infinite;
        transition: 0.25s transform, 0.25s opacity;
        transform: scale(0.875);
        .rd-input-icon-progress-bar-outer {
          position: absolute;
          top: 0;
          left: 0;
          width: 0.375rem;
          height: 0.75rem;
          overflow: hidden;
          .rd-input-icon-progress-bar-inner {
            position: absolute;
            top: 0;
            left: 0;
            width: 0.75rem;
            height: 0.75rem;
            border: 2px solid var(--primary-color);
            border-radius: 50%;
            border-left-color: transparent;
            border-bottom-color: transparent;
            box-sizing: border-box;
            animation: rd-circular-rotate 1000ms cubic-bezier(0.4, 0, 0.22, 1)
              infinite;
          }
        }
      }
    }
    &::before {
      content: "";
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      border-radius: 0.5rem;
      border: var(--border);
      box-sizing: border-box;
      opacity: 0.05;
    }
    &.rd-input-component-disabled {
      pointer-events: none;
      .rd-input-label {
        pointer-events: none;
        filter: grayscale(0.75);
        opacity: 0.5;
      }
    }
  }
</style>
