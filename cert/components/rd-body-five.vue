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
    </div>
    <div class="rd-body-section" rd-last>
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
    <div v-if="data.item.length" class="rd-body-section" rd-items>
      <div v-for="(item, i) in data.item" :key="i" class="rd-body-item">
        <div class="rd-body-item-section">
          <div v-if="item.image" class="rd-body-item-image-container">
            <span class="rd-image-wrapper">
              <span class="rd-image-container rd-image-container-down">
                <img :src="item.image" class="rd-image" />
              </span>
            </span>
          </div>
          <div class="rd-body-item-detail-container">
            <h3 class="rd-body-item-name rd-headline-2">
              <span class="rd-text-wrapper">
                <span class="rd-text-container rd-text-container-down">
                  <span class="rd-text">{{ item.name }}</span>
                </span>
              </span>
            </h3>
          </div>
        </div>
        <div class="rd-body-item-section">
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
        <div class="rd-body-item-border">
          <div class="rd-body-item-border-inner"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  import { ContentBodyFive } from "types/content";

  const props = defineProps<{
    data: ContentBodyFive;
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
      const rdBorder: HTMLElement[] = gsap.utils.toArray(
        rdBody.querySelectorAll(".rd-body-item-border-inner")
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
          rdBorder,
          {
            width: "100%",
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.05,
          },
          "<0"
        )
        .to(
          rdImageContainer,
          {
            y: 0,
            duration: 1,
            ease: "expo.inOut",
            stagger: 0.1,
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
            stagger: 0.1,
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
        padding: 1.5rem 0;
        box-sizing: border-box;
        display: flex;
        align-items: center;
        .rd-body-item-section {
          position: relative;
          width: 50%;
          box-sizing: border-box;
          display: flex;
          align-items: center;
          .rd-body-item-image-container {
            position: relative;
            width: 8rem;
            height: 4rem;
            border-radius: 2rem;
            margin-right: 1rem;
            display: flex;
            overflow: hidden;
          }
          p.rd-body-item-description {
            position: relative;
            width: 100%;
            height: 100%;
            padding: 0 10vw 0 1rem;
            box-sizing: border-box;
            display: flex;
            flex-wrap: wrap;
            justify-content: flex-start;
            align-items: center;
            text-align: left;
            gap: 0.15rem;
          }
          &:first-child {
            padding: 0 1rem 0 0;
          }
          &:last-child {
            padding: 0 0 0 1rem;
            justify-content: flex-end;
          }
        }
        .rd-body-item-border {
          position: absolute;
          bottom: 0;
          left: 0;
          width: 100%;
          height: 1px;
          opacity: 0.25;
          .rd-body-item-border-inner {
            position: relative;
            width: 0;
            height: 100%;
            background: var(--font-primary-color);
          }
        }
        &:last-child {
          .rd-body-item-border {
            display: none;
          }
        }
      }
      &[rd-first] {
        padding: 0 1rem 0 0;
      }
      &[rd-last] {
        padding: 0 10vw 0 1rem;
      }
      &[rd-items] {
        width: 100%;
        margin-top: 1rem;
      }
    }
    @media only screen and (max-width: 1024px) {
      padding: 0 1rem;
      .rd-body-section {
        width: 100%;
        h2.rd-body-headline {
          gap: 0.2rem;
        }
        p.rd-body-description {
          gap: 0.1rem;
        }
        .rd-body-item {
          flex-direction: column;
          .rd-body-item-section {
            width: 100%;
            padding: 0 !important;
            flex-direction: column;
            align-items: flex-start;
            .rd-body-item-image-container {
              width: 100%;
              height: auto;
              aspect-ratio: 2 / 1;
              border-radius: calc(50vw - 1rem);
              margin: 0 0 1rem 0;
            }
            p.rd-body-item-description {
              margin-top: 0.5rem;
              gap: 0.1rem;
              padding: 0;
            }
            &:last-child {
              margin: 0.25rem 0 0 0;
              justify-content: flex-start;
            }
          }
        }
        &[rd-first] {
          padding: 0;
        }
        &[rd-last] {
          margin-top: 0.5rem;
          padding: 0;
        }
        &[rd-items] {
          margin: 0;
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
        .rd-body-item {
          .rd-body-item-section {
            .rd-body-item-image-container {
              margin-right: 0;
              margin-left: 1rem;
            }
            p.rd-body-item-description {
              padding: 0 1rem 0 10vw;
            }
            &:first-child {
              padding: 0 0 0 1rem;
            }
            &:last-child {
              padding: 0 1rem 0 0;
            }
          }
        }
        &[rd-first] {
          padding: 0 0 0 1rem;
        }
        &[rd-last] {
          padding: 0 1rem 0 10vw;
        }
      }
      @media only screen and (max-width: 1024px) {
        .rd-body-section {
          .rd-body-item {
            .rd-body-item-section {
              p.rd-body-item-description {
                padding: 0;
              }
            }
          }
          &[rd-last] {
            padding: 0;
          }
        }
      }
    }
  }
</style>
