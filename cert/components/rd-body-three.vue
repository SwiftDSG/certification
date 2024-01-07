<template>
  <div class="rd-body" ref="rdBody" :rd-section="data.name">
    <span v-if="data.name" class="rd-body-name rd-headline-5">
      <span class="rd-text-wrapper">
        <span class="rd-text-container rd-text-container-down">
          <span class="rd-text">{{ data.name }}</span>
        </span>
      </span>
    </span>
    <div class="rd-body-section" rd-first>
      <h2 class="rd-body-headline rd-title-3">
        <span
          v-for="(word, i) in data.headline"
          :key="i"
          :style="
            word.color !== 'default' ? `color: var(--${word.color}-color);` : ''
          "
          class="rd-word-wrapper"
        >
          <span class="rd-word-container rd-word-container-down">
            <span class="rd-word">{{ word.text }}</span>
          </span>
        </span>
      </h2>
      <p class="rd-body-description rd-subtitle-text">
        <span
          v-for="(text, i) in data.description.split(' ')"
          :key="i"
          class="rd-text-wrapper"
        >
          <span class="rd-text-container rd-text-container-down">
            <span class="rd-text">{{ text }}</span>
          </span>
        </span>
      </p>
    </div>
    <div class="rd-body-section" rd-last>
      <div v-for="(item, i) in data.item" :key="i" class="rd-body-item">
        <div class="rd-body-item-header">
          <div class="rd-body-item-image-container">
            <span class="rd-image-wrapper">
              <span class="rd-image-container rd-image-container-down">
                <img
                  :src="item.image"
                  class="rd-image"
                  :alt="`Item ${item.name}`"
                />
              </span>
            </span>
          </div>
          <div class="rd-body-item-detail-container">
            <span class="rd-body-item-name rd-headline-5">
              <span class="rd-text-wrapper">
                <span class="rd-text-container rd-text-container-down">
                  <span class="rd-text">{{ item.name }}</span>
                </span>
              </span>
            </span>
            <span class="rd-body-item-name rd-caption-text">
              <span class="rd-text-wrapper">
                <span class="rd-text-container rd-text-container-down">
                  <span class="rd-text">{{ item.title }}</span>
                </span>
              </span>
            </span>
          </div>
        </div>
        <div class="rd-body-item-body">
          <p class="rd-body-item-description rd-body-text">
            <span
              v-for="(text, i) in data.description.split(' ')"
              :key="i"
              class="rd-text-wrapper"
            >
              <span class="rd-text-container rd-text-container-down">
                <span class="rd-text">{{ text }}</span>
              </span>
            </span>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  import { ContentBodyThree } from "types/content";

  const props = defineProps<{
    data: ContentBodyThree;
    state: "uninitialized" | "initialized";
  }>();

  const rdBody = ref<HTMLDivElement | null>(null);

  const animate = {
    init(rdBody: HTMLElement): void {
      const tl = gsap.timeline();

      const rdWordContainer: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-word-container")
      );
      const rdWord: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-word")
      );
      const rdTextContainer: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-text-container")
      );
      const rdText: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-text")
      );
      const rdImageContainer: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-image-container")
      );
      const rdImage: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-image")
      );

      tl.to(rdWordContainer, {
        y: 0,
        duration: 1,
        ease: "expo.inOut",
        stagger: 0.05,
      })
        .to(
          rdWord,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.05,
          },
          "<0"
        )
        .to(
          rdTextContainer,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.01,
          },
          "<0.25"
        )
        .to(
          rdText,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.01,
          },
          "<0"
        )
        .to(
          rdImageContainer,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
          },
          "<0"
        )
        .to(
          rdImage,
          {
            y: 0,
            scale: 1,
            duration: 1,
            ease: "expo.inOut",
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
    padding: 0 9rem;
    box-sizing: border-box;
    display: flex;
    flex-shrink: 0;
    flex-wrap: wrap;
    span.rd-body-name {
      position: relative;
      width: 100%;
      display: flex;
      align-items: center;
    }
    .rd-body-section {
      position: relative;
      width: 50%;
      box-sizing: border-box;
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      gap: 1.5rem;
      h2.rd-body-headline {
        position: relative;
        width: 100%;
        padding: 0 2rem 0 0;
        box-sizing: border-box;
        display: flex;
        flex-wrap: wrap;
        gap: 0 0.5rem;
      }
      p.rd-body-description {
        position: relative;
        width: 100%;
        display: flex;
        flex-wrap: wrap;
        gap: 0.2rem;
      }
      .rd-body-item {
        position: relative;
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        .rd-body-item-header {
          position: relative;
          width: 100%;
          display: flex;
          align-items: center;
          gap: 0.75rem;
          .rd-body-item-image-container {
            position: relative;
            width: 2rem;
            height: 2rem;
            border-radius: 0.5rem;
            overflow: hidden;
            * {
              border-radius: 0.5rem;
            }
          }
          .rd-body-item-detail-container {
            position: relative;
            display: flex;
            flex-direction: column;
            align-items: flex-start;
          }
        }
        .rd-body-item-body {
          position: relative;
          width: 100%;
          display: flex;
          p.rd-body-item-description {
            position: relative;
            width: 100%;
            display: flex;
            flex-wrap: wrap;
            gap: 0.15rem;
          }
        }
      }
      &[rd-first] {
        padding: 0 1rem 0 0;
      }
      &[rd-last] {
        padding: 0 10vw 0 1rem;
      }
    }
    @media only screen and (max-width: 1024px) {
      padding: 0 1rem;
      .rd-body-section {
        width: 100%;
        gap: 1rem;

        h2.rd-body-headline {
          gap: 0.2rem;
        }
        p.rd-body-description {
          gap: 0.1rem;
        }
        .rd-body-item {
          .rd-body-item-body {
            p.rd-body-item-description {
              gap: 0.1rem;
            }
          }
        }
        &[rd-first] {
          padding: 0;
        }
        &[rd-last] {
          margin-top: 1rem;
          padding: 0;
        }
      }
    }
  }
  html[dir="rtl"] {
    .rd-body {
      h2.rd-body-headline {
        padding: 0 0 0 2rem;
      }
      .rd-body-section {
        &[rd-first] {
          padding: 0 0 0 1rem;
        }
        &[rd-last] {
          padding: 0 1rem 0 10vw;
        }
      }
      @media only screen and (max-width: 1024px) {
        .rd-body-section {
          &[rd-last] {
            padding: 0;
          }
        }
      }
    }
  }
</style>
