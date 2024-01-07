<template>
  <div class="rd-body">
    <div class="rd-body-marquee">
      <span
        v-for="(text, i) in data.text"
        :key="i"
        class="rd-body-marquee-text rd-title-2"
      >
        {{ text }}
      </span>
      <span
        v-for="(text, i) in data.text"
        :key="i"
        class="rd-body-marquee-text rd-title-2"
      >
        {{ text }}
      </span>
      <span
        v-for="(text, i) in data.text"
        :key="i"
        class="rd-body-marquee-text rd-title-2"
      >
        {{ text }}
      </span>
      <span
        v-for="(text, i) in data.text"
        :key="i"
        class="rd-body-marquee-text rd-title-2"
      >
        {{ text }}
      </span>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import { ContentBodyOne } from "types/content";

  defineProps<{
    data: ContentBodyOne;
  }>();
</script>

<style lang="scss" scoped>
  .rd-body {
    position: relative;
    width: 100vw;
    height: 6rem;
    display: flex;
    overflow: hidden;
    .rd-body-marquee {
      position: relative;
      width: fit-content;
      height: 100%;
      background: var(--font-primary-color);
      box-sizing: border-box;
      display: flex;
      flex-shrink: 0;
      align-items: center;
      animation: rd-marquee 20s linear infinite;
      will-change: transform;
      span.rd-body-marquee-text {
        pointer-events: none;
        position: relative;
        color: var(--primary-color);
        white-space: nowrap;
        text-transform: uppercase;
        display: flex;
        flex-shrink: 0;
        justify-content: center;
        align-items: center;
        &::after {
          content: "";
          position: relative;
          width: 3rem;
          height: 0.375rem;
          margin: 0 1rem;
          background: var(--primary-color);
          flex-shrink: 0;
        }
      }
    }
    @media only screen and (max-width: 1024px) {
      height: 4rem;
      .rd-body-marquee {
        span.rd-body-marquee-text {
          &::after {
            width: 1.5rem;
            height: 0.1875rem;
          }
        }
      }
    }
  }
  html[dir="rtl"] {
    .rd-body {
      .rd-body-marquee {
        animation: rd-marquee-rtl 20s linear infinite;
      }
    }
  }
</style>
