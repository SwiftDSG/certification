<template>
  <div class="rd-body" ref="rdBody">
    <div v-if="data.image" class="rd-body-image">
      <rd-image :state="state" :data="data.image" />
    </div>
    <div class="rd-body-section">
      <div v-for="(item, i) in data.item" :key="i" class="rd-body-item">
        <span class="rd-body-item-value rd-title-2">
          <span class="rd-text-wrapper">
            <span class="rd-text-container rd-text-container-down">
              <span class="rd-text">{{ item.value }}</span>
            </span>
          </span>
        </span>
        <span class="rd-body-item-description rd-subtitle-text">
          <span class="rd-text-wrapper">
            <span class="rd-text-container rd-text-container-down">
              <span class="rd-text">{{ item.description }}</span>
            </span>
          </span>
        </span>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  import { ContentBodyFour } from "types/content";

  const props = defineProps<{
    data: ContentBodyFour;
    state: "uninitialized" | "initialized";
  }>();

  const rdBody = ref<HTMLDivElement | null>(null);

  const animate = {
    init(rdBody: HTMLElement): void {
      const tl = gsap.timeline();

      const rdTextContainer: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-text-container")
      );
      const rdText: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-text")
      );

      tl.to(
        rdTextContainer,
        {
          y: 0,
          duration: 1,
          ease: "expo.inOut",
          stagger: 0.05,
        },
        "<0.25"
      ).to(
        rdText,
        {
          y: 0,
          duration: 1,
          ease: "expo.inOut",
          stagger: 0.05,
        },
        "<0"
      );
    },
  };

  watch(
    () => props.state,
    (val) => {
      if (val === "initialized" && rdBody.value) {
        animate.init(rdBody.value);
      }
    }
  );

  onMounted(() => {
    if (props.state === "initialized" && rdBody.value) {
      animate.init(rdBody.value);
    }
  });
</script>

<style lang="scss" scoped>
  .rd-body {
    position: relative;
    width: 100vw;
    padding: 0 3rem;
    box-sizing: border-box;
    display: flex;
    flex-shrink: 0;
    flex-direction: column;
    gap: 2rem;
    .rd-body-image {
      position: relative;
      width: 100%;
      aspect-ratio: 16 / 9;
      border-radius: 2rem;
      display: flex;
      overflow: hidden;
    }
    .rd-body-section {
      position: relative;
      width: 100%;
      padding: 0 6rem;
      box-sizing: border-box;
      display: flex;
      justify-content: center;
      .rd-body-item {
        position: relative;
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
      }
    }
    @media only screen and (max-width: 1024px) {
      padding: 0 1rem;
      gap: 1rem;
      .rd-body-section {
        width: 100%;
        padding: 0;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        .rd-body-item {
          justify-content: flex-start;
          align-items: flex-start;
        }
      }
    }
  }
</style>
