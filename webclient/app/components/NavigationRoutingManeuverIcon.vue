<script setup lang="ts">
import type { components } from "~/api_types";
import {
  ArrowLeftEndOnRectangleIcon,
  ArrowLeftStartOnRectangleIcon,
  ArrowTurnUpLeftIcon,
  ArrowTurnUpRightIcon,
  ArrowUpIcon,
  ArrowLeftIcon,
  ArrowRightIcon,
  MapPinIcon,
  ArrowsUpDownIcon,
  TicketIcon,
  ChevronDoubleLeftIcon,
  ChevronDoubleRightIcon,
  ChevronDoubleDownIcon,
  ArrowUpRightIcon,
  ArrowUpLeftIcon,
  ArrowPathIcon,
  ArrowUturnRightIcon,
  ArrowUturnLeftIcon,
  ArrowUpCircleIcon,
} from "@heroicons/vue/24/outline";

type ManeuverTypeResponse = components["schemas"]["ManeuverTypeResponse"];
defineProps<{ type: ManeuverTypeResponse }>();
</script>

<template>
  <div class="min-w-11 rounded-full p-2 text-zinc-900">
    <ArrowLeftEndOnRectangleIcon v-if="type === 'building_exit' || type === 'exit_left'" class="mx-auto h-7 w-7" />
    <ArrowLeftEndOnRectangleIcon v-else-if="type === 'exit_right'" class="mx-auto h-7 w-7 scale-x-[-1]" />
    <ArrowLeftStartOnRectangleIcon v-else-if="type === 'building_enter'" class="mx-auto h-7 w-7" />
    <ArrowsUpDownIcon
      v-else-if="type === 'elevator_enter' || type === 'escalator_enter' || type === 'steps_enter'"
      class="mx-auto h-7 w-7"
    />

    <TicketIcon
      v-else-if="
        type === 'transit' ||
        type === 'transit_transfer' ||
        type === 'transit_remain_on' ||
        type === 'transit_connection_start' ||
        type === 'transit_connection_transfer' ||
        type === 'transit_connection_destination' ||
        type === 'post_transit_connection_destination' ||
        type === 'ferry_enter' ||
        type === 'ferry_exit'
      "
      class="mx-auto h-7 w-7"
    />

    <ArrowTurnUpLeftIcon v-else-if="type === 'left' || type === 'sharp_left'" class="mx-auto h-7 w-7" />
    <ArrowTurnUpRightIcon v-else-if="type === 'right' || type === 'sharp_right'" class="mx-auto h-7 w-7" />
    <ArrowUpIcon
      v-else-if="type === 'start' || type === 'becomes' || type === 'stay_straight' || type === 'ramp_straight'"
      class="mx-auto h-7 w-7"
    />
    <ChevronDoubleLeftIcon v-else-if="type === 'merge_left'" class="mx-auto h-7 w-7" />
    <ChevronDoubleRightIcon v-else-if="type === 'merge_right'" class="mx-auto h-7 w-7" />
    <ChevronDoubleDownIcon v-else-if="type === 'merge'" class="mx-auto h-7 w-7" />
    <ArrowUpRightIcon
      v-else-if="type === 'slight_right' || type === 'stay_right' || type === 'ramp_right'"
      class="mx-auto h-7 w-7"
    />
    <ArrowUpLeftIcon
      v-else-if="type === 'slight_left' || type === 'stay_left' || type === 'ramp_left'"
      class="mx-auto h-7 w-7"
    />
    <ArrowPathIcon v-else-if="type === 'roundabout_enter' || type === 'roundabout_exit'" class="mx-auto h-7 w-7" />
    <ArrowUturnRightIcon v-else-if="type === 'uturn_right'" class="mx-auto h-7 w-7" />
    <ArrowUturnLeftIcon v-else-if="type === 'uturn_left'" class="mx-auto h-7 w-7" />
    <ArrowUpCircleIcon v-else-if="type === 'continue'" class="mx-auto h-7 w-7" />
    <ArrowLeftIcon v-else-if="type === 'start_left'" class="mx-auto h-7 w-7" />
    <ArrowRightIcon v-else-if="type === 'start_right'" class="mx-auto h-7 w-7" />
    <MapPinIcon
      v-else-if="type === 'destination' || type === 'destination_left' || type === 'destination_right'"
      class="mx-auto h-7 w-7"
    />
    <div v-else>{{ type }}</div>
  </div>
</template>
