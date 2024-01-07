<template>
  <div
    ref="rdInputComponent"
    class="rd-input-component"
    :class="`${input.error ? 'rd-input-error' : ''} ${
      dropDownState === 'opened' ? 'rd-input-opened' : ''
    } ${input.disabled ? 'rd-input-disabled' : ''}`"
    @focusout="focusHandler"
  >
    <label v-if="input.label" class="rd-input-label rd-headline-6" :for="id">{{
      input.label
    }}</label>
    <div class="rd-input-container">
      <div v-if="input.icon" class="rd-input-icon-container">
        <rd-svg :name="input.icon" :color="'secondary'" />
      </div>
      <input
        class="rd-input rd-body-text"
        ref="rdInput"
        type="date"
        :name="input.name"
        :id="id"
        :min="inputMin"
        :max="inputMax"
        :class="!inputModel.value ? 'rd-input-empty' : ''"
        placeholder="Select date"
        :disabled="input.disabled"
        @input="updateModel"
        @keydown.space.prevent="dropDownHandler('open')"
      />
      <div class="rd-input-border"></div>
      <div
        class="rd-input-calendar-container"
        @click="dropDownHandler('open')"
        @keypress.space.enter.prevent="dropDownHandler('open')"
        tabindex="0"
      >
        <rd-svg class="rd-input-calendar" name="calendar" />
      </div>
    </div>
    <rd-popover
      v-if="inputOptions.length"
      class="rd-input-option"
      :state="dropDownState"
      @exit="removeFocus"
    >
      <div class="rd-input-option-controls-container">
        <div class="rd-input-option-controls-indicator-wrapper">
          <span class="rd-input-option-controls-indicator rd-headline-5">
            {{
              `${months[inputOptions[dropDownIndex].month]} ${
                inputOptions[dropDownIndex].year
              }`
            }}
          </span>
          <span
            v-if="inputOptionDecoy"
            ref="rdInputOptionControlsIndicatorDecoy"
            class="rd-input-option-controls-indicator rd-input-option-controls-indicator-decoy rd-headline-5"
          >
            {{ `${months[inputOptionDecoy.month]} ${inputOptionDecoy.year}` }}
          </span>
        </div>
        <div class="rd-input-option-controls">
          <rd-input-button-small
            class="rd-panel-button"
            side="left"
            :icon="'chevron-left'"
            :disabled="dropDownIndex === 0"
            style="transform-origin: center right"
            @clicked="changeMonth('left')"
          />
          <rd-input-button-small
            class="rd-panel-button"
            side="right"
            :icon="'chevron-right'"
            :disabled="dropDownIndex === inputOptions.length - 1"
            style="transform-origin: center left"
            @clicked="changeMonth('right')"
          />
        </div>
      </div>
      <div class="rd-input-option-days-container">
        <span class="rd-input-option-day rd-caption-text">S</span>
        <span class="rd-input-option-day rd-caption-text">M</span>
        <span class="rd-input-option-day rd-caption-text">T</span>
        <span class="rd-input-option-day rd-caption-text">W</span>
        <span class="rd-input-option-day rd-caption-text">T</span>
        <span class="rd-input-option-day rd-caption-text">F</span>
        <span class="rd-input-option-day rd-caption-text">S</span>
      </div>
      <div class="rd-input-option-weeks-wrapper">
        <div
          class="rd-input-option-weeks"
          ref="rdInputOptionWeeks"
          :class="dropDownChanging ? 'rd-input-option-weeks-changing' : ''"
        >
          <div v-for="i in 6" :key="i" class="rd-input-option-week">
            <div
              v-for="j in 7"
              :key="j"
              class="rd-input-option-week-day"
              :class="`${
                inputOptions[dropDownIndex].dates[i - 1][j - 1]
                  ? inputOptions[dropDownIndex].dates[i - 1][j - 1]?.option
                      ?.map((a) => `rd-input-option-week-day-${a}`)
                      .join(' ')
                  : 'rd-input-option-week-day-disabled'
              }`"
              :tabindex="
                dropDownState === 'opened' &&
                inputOptions[dropDownIndex].dates[i - 1][j - 1] &&
                !inputOptions[dropDownIndex].dates[i - 1][
                  j - 1
                ]?.option?.includes('disabled')
                  ? '0'
                  : '-1'
              "
              @click="selectOption(i - 1, j - 1)"
              @keypress.enter="selectOption(i - 1, j - 1)"
              :style="
                !inputOptions[dropDownIndex].dates[i - 1][j - 1]
                  ? 'pointer-events: none'
                  : ''
              "
            >
              <span
                v-if="inputOptions[dropDownIndex].dates[i - 1][j - 1]"
                class="rd-input-option-week-day-date rd-headline-6"
                >{{ inputOptions[dropDownIndex].dates[i - 1][j - 1]?.date }}
              </span>
              <span v-else class="rd-input-option-week-day-dot"></span>
            </div>
          </div>
        </div>
        <div
          v-if="inputOptionDecoy"
          ref="rdInputOptionWeeksDecoy"
          class="rd-input-option-weeks rd-input-option-weeks-decoy"
        >
          <div v-for="i in 6" :key="i" class="rd-input-option-week">
            <div
              v-for="j in 7"
              :key="j"
              class="rd-input-option-week-day"
              :class="`${
                inputOptionDecoy.dates[i - 1][j - 1]
                  ? inputOptionDecoy.dates[i - 1][j - 1]?.option
                      ?.map((a) => `rd-input-option-week-day-${a}`)
                      .join(' ')
                  : 'rd-input-option-week-day-disabled'
              }`"
            >
              <span
                v-if="inputOptionDecoy.dates[i - 1][j - 1]"
                class="rd-input-option-week-day-date rd-headline-6"
                >{{ inputOptionDecoy.dates[i - 1][j - 1]?.date }}
              </span>
              <span v-else class="rd-input-option-week-day-dot"></span>
            </div>
          </div>
        </div>
      </div>
    </rd-popover>
    <span
      v-if="typeof input.error === 'string'"
      class="rd-input-error rd-headline-6"
    >
      <span class="rd-text-wrapper">
        <span class="rd-text-container rd-text-container-up">
          <span class="rd-text">{{ inputError }}</span>
        </span>
      </span>
    </span>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { InputDateOption } from "~~/types/general.js";

  type Selection = {
    year: number;
    month: number;
    dates: (SelectionDate | null)[][];
  };
  type SelectionDate = {
    date: number;
    option?: ("disabled" | "selected" | "today")[];
  };

  const props = defineProps<{
    input: InputDateOption;
  }>();
  const emits = defineEmits(["focusout"]);
  const { view } = useMain();

  const rdInputOptionWeeks = ref<HTMLDivElement | null>(null);
  const rdInputOptionWeeksDecoy = ref<HTMLDivElement | null>(null);
  const rdInputOptionControlsIndicatorDecoy = ref<HTMLDivElement | null>(null);
  const rdInputComponent = ref<HTMLInputElement | null>(null);
  const rdInput = ref<HTMLInputElement | null>(null);

  const inputError = ref<string | undefined>(props.input.error);
  const inputModel = ref<InputDateOption["model"]>({
    name: "",
    value: 0,
  });
  const inputMin = ref<string>("");
  const inputMax = ref<string>("");
  const inputOptions = ref<Selection[]>([]);
  const inputOptionDecoy = ref<Selection | null>(null);
  const inputIndex = ref<[number, number, number]>([-1, -1, -1]);
  const inputFocusIndex = ref<number>(-1);

  const dropDownState = ref<"opened" | "closed">("closed");
  const dropDownIndex = ref<number>(-1);
  const dropDownChanging = ref<boolean>(false);
  const dropDownOptions = ref<{
    minIndex: number;
    maxIndex: number;
    rdOptions: HTMLElement[];
  }>({
    minIndex: -1,
    maxIndex: -1,
    rdOptions: [],
  });

  const id = computed<string>(() => {
    const characters =
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let str = "";
    for (let i = 0; i < 10; i++) {
      str += characters[Math.round(Math.random() * (characters.length - 1))];
    }

    return `rd-input-${str}`;
  });
  const today = computed<Date>(() => new Date());

  const months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  function focusHandler(e: FocusEvent): void {
    if (
      !e.relatedTarget ||
      (e.relatedTarget instanceof HTMLElement &&
        !rdInputComponent.value?.contains(e.relatedTarget))
    ) {
      dropDownHandler("close");
      emits("focusout");
    }
  }
  function keyHandler(e: KeyboardEvent): void {
    if (e.key.includes("Arrow") && rdInputOptionWeeks.value) {
      e.preventDefault();
      let changing = false;

      if (inputFocusIndex.value === -1) {
        inputFocusIndex.value = dropDownOptions.value.minIndex;
      } else if (e.key === "ArrowUp" || e.key === "ArrowLeft") {
        if (inputFocusIndex.value === dropDownOptions.value.minIndex) {
          changing = true;
          changeMonth("left", true);
        } else if (e.key === "ArrowUp") {
          inputFocusIndex.value =
            inputFocusIndex.value - dropDownOptions.value.minIndex <= 7
              ? dropDownOptions.value.minIndex
              : inputFocusIndex.value - 7;
        } else {
          for (let i = inputFocusIndex.value - 1; i >= 0; i--) {
            if (
              dropDownOptions.value.rdOptions[i] &&
              !dropDownOptions.value.rdOptions[i].classList.contains(
                "rd-input-option-week-day-disabled"
              )
            ) {
              inputFocusIndex.value = i;
              break;
            }
          }
        }
      } else {
        if (inputFocusIndex.value === dropDownOptions.value.maxIndex) {
          changing = true;
          changeMonth("right", true);
        } else if (e.key === "ArrowDown") {
          inputFocusIndex.value =
            inputFocusIndex.value >= dropDownOptions.value.maxIndex - 7
              ? dropDownOptions.value.maxIndex
              : inputFocusIndex.value + 7;
        } else {
          for (
            let i = inputFocusIndex.value + 1;
            i < dropDownOptions.value.rdOptions.length;
            i++
          ) {
            if (
              dropDownOptions.value.rdOptions[i] &&
              !dropDownOptions.value.rdOptions[i].classList.contains(
                "rd-input-option-week-day-disabled"
              )
            ) {
              inputFocusIndex.value = i;
              break;
            }
          }
        }
      }
      if (!changing) {
        const rdInputOptionWeekDay =
          dropDownOptions.value.rdOptions[inputFocusIndex.value];
        if (rdInputOptionWeekDay instanceof HTMLElement) {
          rdInputOptionWeekDay.focus();
        }
      } else {
        inputFocusIndex.value = dropDownOptions.value.minIndex;
      }
    } else if (e.key === "Escape") {
      removeFocus();
    }
  }
  function dropDownHandler(state: "open" | "close"): void {
    if (state === "open") {
      dropDownState.value = "opened";
      if (view.value === "desktop")
        window.addEventListener("keydown", keyHandler);
    } else {
      window.removeEventListener("keydown", keyHandler);
      inputFocusIndex.value = 0;
      dropDownState.value = "closed";
    }
  }
  function removeFocus(): void {
    inputFocusIndex.value = 0;
    dropDownState.value = "closed";
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }
  function updateModel(e: Event): void {
    if (e.target instanceof HTMLInputElement) {
      inputModel.value = {
        name: e.target.value,
        value: new Date(e.target.value).setHours(0, 0, 0, 0),
      };
      props.input.model = inputModel.value;
    }
  }
  function selectOption(i: number, j: number): void {
    if (!inputIndex.value.includes(-1)) {
      const [i, j, k] = inputIndex.value;
      const index = (inputOptions.value[i].dates[j][k]?.option || []).indexOf(
        "selected"
      );
      if (index > -1) {
        inputOptions.value[i].dates[j][k]?.option?.splice(index, 1);
      }
    }
    inputIndex.value = [parseInt(dropDownIndex.value.toString()), i, j];
    if (
      inputOptions.value[dropDownIndex.value].dates[i][j]?.option &&
      rdInput.value
    ) {
      inputOptions.value[dropDownIndex.value].dates[i][j]?.option?.push(
        "selected"
      );
      rdInput.value.value = `${inputOptions.value[dropDownIndex.value].year}-${(
        inputOptions.value[dropDownIndex.value].month + 1
      )
        .toString()
        .padStart(2, "0")}-${inputOptions.value[dropDownIndex.value].dates[i][
        j
      ]?.date
        .toString()
        .padStart(2, "0")}`;
      inputModel.value = {
        name: rdInput.value.value,
        value: new Date(rdInput.value.value).setHours(0, 0, 0, 0),
      };
      props.input.model = inputModel.value;
    }
    inputFocusIndex.value = dropDownOptions.value.minIndex;
  }

  async function changeMonth(
    dir: "left" | "right",
    focus?: boolean
  ): Promise<void> {
    if (
      dir === "right" &&
      dropDownIndex.value < inputOptions.value.length - 1
    ) {
      inputOptionDecoy.value = inputOptions.value[dropDownIndex.value + 1];
    } else if (dir === "left" && dropDownIndex.value > 0) {
      inputOptionDecoy.value = inputOptions.value[dropDownIndex.value - 1];
    }

    await nextTick();

    if (
      rdInputOptionWeeksDecoy.value &&
      rdInputOptionControlsIndicatorDecoy.value &&
      !dropDownChanging.value
    ) {
      dropDownChanging.value = true;

      const rdInputOption =
        rdInputOptionWeeksDecoy.value.previousElementSibling;
      const rdInputOptionControlsIndicator =
        rdInputOptionControlsIndicatorDecoy.value.previousElementSibling;

      const tl = gsap.timeline({
        onComplete() {
          if (dir === "right") {
            if (
              dropDownIndex.value === inputOptions.value.length - 2 &&
              (!props.input.threshold?.max ||
                new Date(
                  inputOptions.value[inputOptions.value.length - 1].year,
                  inputOptions.value[inputOptions.value.length - 1].month,
                  32
                ).setDate(1) <= props.input.threshold.max.getTime())
            ) {
              const tm =
                inputOptions.value[inputOptions.value.length - 1].month === 11
                  ? 0
                  : inputOptions.value[inputOptions.value.length - 1].month + 1;
              const ty =
                inputOptions.value[inputOptions.value.length - 1].year +
                (tm === 0 ? 1 : 0);
              inputOptions.value.push({
                year: ty,
                month: tm,
                dates: getDates(ty, tm),
              });
            }
            dropDownIndex.value++;
          } else {
            if (
              dropDownIndex.value === 1 &&
              (!props.input.threshold?.min ||
                new Date(
                  inputOptions.value[0].year,
                  inputOptions.value[0].month,
                  1
                ).setHours(0, 0, 0, 0) -
                  1 >=
                  props.input.threshold.min.getTime())
            ) {
              const ym =
                inputOptions.value[0].month === 0
                  ? 11
                  : inputOptions.value[0].month - 1;
              const yy = inputOptions.value[0].year - (ym === 11 ? 1 : 0);
              if (!inputIndex.value.includes(-1)) inputIndex.value[0]++;
              inputOptions.value.unshift({
                year: yy,
                month: ym,
                dates: getDates(yy, ym),
              });
            } else {
              dropDownIndex.value--;
            }
          }
          inputOptionDecoy.value = null;
          gsap.to(rdInputOptionControlsIndicator, {
            y: 0,
            duration: 0,
          });
          gsap.to(rdInputOption, {
            x: 0,
            duration: 0,
          });

          setOptions(focus);
          if (!focus) dropDownChanging.value = false;
        },
      });

      tl.to(rdInputOptionControlsIndicatorDecoy.value, {
        top: dir === "right" ? "100%" : "-100%",
        duration: 0,
      })
        .to(
          [
            rdInputOptionControlsIndicatorDecoy.value,
            rdInputOptionControlsIndicator,
          ],
          {
            y: dir === "right" ? "-100%" : "100%",
            duration: 0.25,
          }
        )
        .to(
          rdInputOptionWeeksDecoy.value,
          {
            left: dir === "right" ? "100%" : "-100%",
            duration: 0,
          },
          "<0"
        )
        .to(
          [rdInputOptionWeeksDecoy.value, rdInputOption],
          {
            x: dir === "right" ? "-100%" : "100%",
            duration: 0.25,
          },
          "<0"
        );
    } else {
      if (dir === "right")
        inputFocusIndex.value = dropDownOptions.value.maxIndex;
      else inputFocusIndex.value = dropDownOptions.value.minIndex;
    }
  }
  async function setOptions(focus?: boolean): Promise<void> {
    await nextTick();

    dropDownOptions.value.rdOptions = gsap.utils.toArray(
      rdInputOptionWeeks.value?.querySelectorAll(".rd-input-option-week-day") ||
        []
    );
    let found = false;
    const length = dropDownOptions.value.rdOptions.length;
    for (let i = 0; i < length; i++) {
      if (
        !dropDownOptions.value.rdOptions[i].classList.contains(
          "rd-input-option-week-day-disabled"
        )
      ) {
        if (!found) {
          dropDownOptions.value.minIndex = i;
          found = true;
        }
        dropDownOptions.value.maxIndex = i;
      }
      if (i === length - 1 && focus) {
        inputFocusIndex.value = dropDownOptions.value.minIndex;
        dropDownOptions.value.rdOptions[dropDownOptions.value.minIndex].focus();
        dropDownChanging.value = false;
      }
    }
  }

  function getMonthDates(y: number, m: number): number {
    m++;
    return m === 2
      ? y & 3 || (!(y % 25) && y & 15)
        ? 28
        : 29
      : 30 + ((m + (m >> 3)) & 1);
  }
  function getDates(y: number, m: number): (SelectionDate | null)[][] {
    let date = 1;
    const first = new Date(y, m).getDay();
    const dates: (SelectionDate | null)[][] = [];
    for (let i = 0; i < 6; i++) {
      const week: (SelectionDate | null)[] = [];
      for (let j = 0; j < 7; j++) {
        if ((i === 0 && j < first) || date > getMonthDates(y, m)) {
          week.push(null);
        } else {
          let option: SelectionDate["option"] = [];

          if (
            y === today.value.getFullYear() &&
            m === today.value.getMonth() &&
            date === today.value.getDate()
          )
            option.push("today");
          if (
            props.input.threshold &&
            ((props.input.threshold.min &&
              props.input.threshold.min.getTime() >
                new Date(y, m, date).setHours(0, 0, 0, 0)) ||
              (props.input.threshold.max &&
                props.input.threshold.max.getTime() <
                  new Date(y, m, date).setHours(0, 0, 0, 0)) ||
              (props.input.threshold.day?.length &&
                !props.input.threshold.day.includes(
                  new Date(y, m, date).getDay()
                )))
          ) {
            option.push("disabled");
          }
          week.push({
            date,
            option,
          });
          date++;
        }
      }
      dates.push(week);
    }
    return dates;
  }

  watch(
    () => props.input.error,
    (val) => {
      if (val) inputError.value = val;
    }
  );

  onBeforeUnmount(() => {
    window.removeEventListener("keydown", keyHandler);
  });
  onMounted(() => {
    const date = props.input.model.value
      ? new Date(new Date(props.input.model.value).setHours(0, 0, 0, 0))
      : new Date();

    let cy = date.getFullYear();
    let cm = date.getMonth();
    let ym = date.getMonth() - 1 < 0 ? 11 : date.getMonth() - 1;
    let yy = cy - (ym === 11 ? 1 : 0);
    let tm = date.getMonth() + 1 > 11 ? 0 : date.getMonth() + 1;
    let ty = cy + (tm === 0 ? 1 : 0);

    dropDownIndex.value = 1;

    if (props.input.threshold?.min || props.input.threshold?.max) {
      if (props.input.threshold.min) {
        inputMin.value = props.input.threshold.min.toISOString().slice(0, 10);
        dropDownIndex.value = 0;
        if (ym !== props.input.threshold.min.getMonth()) {
          ym = props.input.threshold.min.getMonth();
          yy = props.input.threshold.min.getFullYear();
        }
        inputOptions.value.push({
          year: yy,
          month: ym,
          dates: getDates(yy, ym),
        });
        if (
          !props.input.threshold.max ||
          props.input.threshold.max.getFullYear() >= yy ||
          props.input.threshold.max.getMonth() > ym
        ) {
          if (ym === 11) {
            cm = 0;
            cy = yy + 1;
          } else {
            cm = ym + 1;
            cy = yy;
          }
        }
        inputOptions.value.push({
          year: cy,
          month: cm,
          dates: getDates(cy, cm),
        });
      } else if (props.input.threshold.max) {
        inputMax.value = props.input.threshold.max.toISOString().slice(0, 10);
        dropDownIndex.value = 1;
        if (tm !== props.input.threshold.max.getMonth()) {
          tm = props.input.threshold.max.getMonth();
          ty = props.input.threshold.max.getFullYear();
        }
        inputOptions.value.push({
          year: ty,
          month: tm,
          dates: getDates(ty, tm),
        });
        if (
          !props.input.threshold.max ||
          props.input.threshold.max.getFullYear() >= yy ||
          props.input.threshold.max.getMonth() > ym
        ) {
          if (tm === 0) {
            cm = 11;
            cy = ty - 1;
          } else {
            cm = tm - 1;
            cy = ty;
          }
        }
        inputOptions.value.unshift({
          year: cy,
          month: cm,
          dates: getDates(cy, cm),
        });
      }
    } else {
      inputOptions.value.push(
        {
          year: yy - (ym === 11 ? 1 : 0),
          month: ym,
          dates: getDates(yy, ym),
        },
        {
          year: cy,
          month: cm,
          dates: getDates(cy, cm),
        },
        {
          year: ty + (tm === 0 ? 1 : 0),
          month: tm,
          dates: getDates(ty, tm),
        }
      );
    }

    setOptions();
  });
</script>

<style lang="scss" scoped>
  .rd-input-component {
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-content: flex-start;
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
      .rd-input-icon-container {
        position: relative;
        width: 2rem;
        height: 2rem;
        padding: 0 0.5rem;
        border-top-left-radius: 0.5rem;
        border-bottom-left-radius: 0.5rem;
        box-sizing: border-box;
        background: rgba(0, 0, 0, 0.05);
        display: flex;
        justify-content: center;
        align-items: center;
      }
      input.rd-input {
        position: relative;
        width: 100%;
        height: 100%;
        padding: 0 0.5rem;
        border: none;
        border-radius: 0.5rem;
        box-sizing: border-box;
        color: var(--font-primary-color);
        background: rgba(0, 0, 0, 0);
        display: flex;
        transition: background-color 0.25s;
        &::-webkit-calendar-picker-indicator {
          position: absolute;
          opacity: 0;
          display: none;
        }
        &.rd-input-empty {
          color: var(--font-primary-color);
          opacity: 0.5;
          transition: opacity 0.25s;
        }
        &::placeholder {
          color: var(--font-primary-color);
          opacity: 0.5;
          transition: opacity 0.25s;
        }
        &:hover {
          outline: none;
          background: rgba(0, 0, 0, 0);
          &.rd-input-empty {
            opacity: 1;
          }
          &::placeholder {
            opacity: 1;
          }
        }
        &:focus,
        &.rd-input-focused {
          outline: none;
          background: var(--background-depth-two-color);
        }
        &:focus + .rd-input-border,
        &.rd-input-focused + .rd-input-border {
          border-color: var(--primary-color);
          &::before {
            opacity: 0.25;
          }
        }
      }
      .rd-input-calendar-container {
        cursor: pointer;
        position: absolute;
        top: 0;
        right: 0;
        width: 2rem;
        height: 2rem;
        border-top-right-radius: 0.5rem;
        border-bottom-right-radius: 0.5rem;
        padding: 0 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
      }
      .rd-input-border {
        z-index: 2;
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
          border-radius: 0.5rem;
          border: 3px solid var(--primary-color);
          box-sizing: border-box;
          opacity: 0;
          transition: 0.25s opacity;
        }
      }
    }
    .rd-input-option {
      top: calc(100% + 2px);
      width: calc(15.5rem + 2px);
      .rd-input-option-controls-container {
        position: relative;
        width: 100%;
        height: 2.5rem;
        padding: 0.5rem 0.5rem 0;
        box-sizing: border-box;
        display: flex;
        justify-content: space-between;
        align-items: center;
        .rd-input-option-controls-indicator-wrapper {
          position: relative;
          width: calc(100% - 4rem);
          height: 100%;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: flex-start;
          overflow: hidden;
          .rd-input-option-controls-indicator {
            position: relative;
            width: 100%;
            height: 100%;
            white-space: nowrap;
            display: flex;
            flex-shrink: 0;
            align-items: center;
            &.rd-input-option-controls-indicator-decoy {
              position: absolute;
              top: 100%;
            }
          }
        }
        .rd-input-option-controls {
          position: relative;
          height: 100%;
          display: flex;
        }
      }
      .rd-input-option-days-container {
        position: relative;
        width: 100%;
        margin: 0.75rem 0;
        padding: 0 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 0.5rem;
        span.rd-input-option-day {
          position: relative;
          width: 1.5rem;
          height: 1rem;
          display: flex;
          justify-content: center;
          align-items: center;
        }
      }
      .rd-input-option-weeks-wrapper {
        position: relative;
        width: 100%;
        display: flex;
        flex-shrink: 0;
        justify-content: center;
        .rd-input-option-weeks {
          position: relative;
          width: 100%;
          padding: 0 0.5rem;
          box-sizing: border-box;
          display: flex;
          flex-shrink: 0;
          flex-direction: column;
          gap: 0.5rem;
          .rd-input-option-week {
            position: relative;
            width: 100%;
            height: 1.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            .rd-input-option-week-day {
              cursor: pointer;
              position: relative;
              width: 1.5rem;
              height: 1.5rem;
              border-radius: 0.5rem;
              border: 1px solid transparent;
              box-sizing: border-box;
              display: flex;
              justify-content: center;
              align-items: center;
              * {
                pointer-events: none;
              }
              &::before {
                content: "";
                position: absolute;
                width: 100%;
                height: 100%;
                border-radius: 0.5rem;
                background: #000;
                opacity: 0;
                transition: background-color 0.25s, opacity 0.25s;
              }
              span.rd-input-option-week-day-date {
                cursor: pointer;
                position: relative;
                width: 100%;
                height: 100%;
                line-height: 1;
                opacity: 0.5;
                display: flex;
                justify-content: center;
                align-items: center;
              }
              span.rd-input-option-week-day-dot {
                pointer-events: none;
                position: relative;
                width: 6px;
                height: 6px;
                border-radius: 3px;
                background: var(--font-primary-color);
                opacity: 0.2;
              }
              &.rd-input-option-week-day-disabled {
                pointer-events: none;
                opacity: 0.5;
              }
              &.rd-input-option-week-day-selected {
                background: var(--primary-color) !important;
                span.rd-input-option-week-day-date {
                  color: #fff !important;
                  opacity: 1 !important;
                }
              }
              &.rd-input-option-week-day-today {
                border-color: var(--primary-color);
                span.rd-input-option-week-day-date {
                  color: var(--primary-color);
                  opacity: 1;
                }
              }

              &:hover,
              &:focus {
                outline: none;
                &::before {
                  opacity: 0.05;
                }
              }
            }
          }
          &.rd-input-option-weeks-changing {
            .rd-input-option-week {
              .rd-input-option-week-day:focus {
                &::before {
                  opacity: 0 !important;
                }
              }
            }
          }
          &.rd-input-option-weeks-decoy {
            position: absolute;
            top: 0;
            left: 100%;
          }
        }
      }
    }
    span.rd-input-error {
      position: relative;
      width: 100%;
      height: 1rem;
      color: var(--error-color);
      white-space: nowrap;
      display: flex;
      justify-content: flex-start;
      align-items: center;
      flex-wrap: nowrap;
      span.rd-text-wrapper {
        overflow: visible;
        justify-content: flex-start;
        span.rd-text-container {
          justify-content: flex-start;
          transition: 0.25s transform;
          span.rd-text {
            transition: 0.25s transform;
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
    &:focus-within,
    &.rd-input-opened {
      .rd-input-container {
        input.rd-input {
          outline: none;
          background: var(--background-depth-two-color);
          & + .rd-input-border {
            border-color: var(--primary-color);
            &::before {
              opacity: 0.25;
            }
          }
        }
      }
    }
    @media only screen and (max-width: 1024px) {
      .rd-input-option {
        width: 100vw;
      }
    }
  }
</style>
