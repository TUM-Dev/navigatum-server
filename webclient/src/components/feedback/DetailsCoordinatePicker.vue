<script setup lang="ts">
import { selectedMap, useDetailsStore } from "@/stores/details";
import { Coord, useGlobalStore } from "@/stores/global";
import { ref } from "vue";
import { getLocalStorageWithExpiry, setLocalStorageWithExpiry } from "@/composables/storage";
const state = useDetailsStore();
const global = useGlobalStore();
// The coordinate picker keeps backups of the subject and body
// in case someone writes a text and then after that clicks
// the set coordinate button in the feedback form.
// If we no backup has been made then, this would be lost after clicking confirm there.
const coord_picker = ref({
  backup_id: null as string | null,
  subject_backup: null as string | null,
  body_backup: null as string | null,
  force_reopen: false,
});
const emit = defineEmits<{
  (e: "openFeedbackForm", callback: EventListener): void;
}>();

function addLocationPicker() {
  // If this is called from the feedback form using the edit coordinate
  // button, we temporarily save the current subject and body, so it is
  // not lost when being reopened
  if (global.feedback.open) {
    coord_picker.value.backup_id = state.data?.id || "undefined";
    coord_picker.value.subject_backup = global.feedback.subject;
    coord_picker.value.body_backup = global.feedback.body;
    coord_picker.value.force_reopen = true; // reopen after confirm

    global.temporarilyCloseFeedback();
  }

  state.map.selected = selectedMap.interactive;

  // Verify that there isn't already a marker (could happen if you click 'assign
  // a location' multiple times from the 'missing accurate location' toast)
  if (marker2.value === null) {
    // Coordinates are either taken from the entry, or if there are already
    // some in the localStorage use them
    const currentEdits = getLocalStorageWithExpiry<{ [index: string]: Coord }>("feedback-coords", {});

    const { coords } = currentEdits[state.data?.id || "undefined"] || state.data;
    marker2.value = new Marker({
      draggable: true,
      color: "#ff0000",
    });
    if (coords.lat !== undefined && coords.lon !== undefined)
      marker2.value.setLngLat([coords.lon, coords.lat]).addTo(map.value as Map);
  }
}
function confirmLocationPicker() {
  // add the current edits to the feedback
  const currentEdits = getLocalStorageWithExpiry<{ [index: string]: Coord }>("feedback-coords", {});
  const location = marker2.value?.getLngLat();
  currentEdits[state.data?.id || "undefined"] = {
    coords: { lat: location?.lat, lon: location?.lng },
  };
  // save to local storage with ttl of 12h (garbage-collected on next read)
  setLocalStorageWithExpiry("feedback-coords", currentEdits, 12);

  marker2.value?.remove();
  marker2.value = null;

  // A feedback form is only opened when this is the only (and therefore
  // first coordinate). If there are more coordinates we can assume
  // someone is doing batch edits. They can then use the send button in
  // the coordinate counter at the top of the page.
  if (Object.keys(currentEdits).length === 1 || state.coord_picker.force_reopen) {
    state.coord_picker.force_reopen = false;
    emit("openFeedbackForm", () => addLocationPicker());
  }

  // The helptext (which says thet you can edit multiple coordinates in bulk)
  // is also only shown if there is one edit.
  if (Object.keys(currentEdits).length === 1) {
    document.getElementById("feedback-coordinate-picker-helptext")?.classList.remove("d-none");
  }
}
function cancelLocationPicker() {
  marker2.value?.remove();
  marker2.value = null;

  if (state.coord_picker.force_reopen) {
    state.coord_picker.force_reopen = false;
    emit("openFeedbackForm", () => addLocationPicker());
  }
}
</script>
<template>
  <Teleport to="maybe-coordinate-inacurate-warning-toast">
    <div class="toast toast-warning" v-if="state.data?.coords.accuracy === 'building'">
      {{ $t("view_view.msg.inaccurate_only_building.primary_msg") }}<br />
      <i>
        {{ $t("view_view.msg.inaccurate_only_building.help_others_and") }}
        <button class="btn btn-sm" @click="addLocationPicker">
          {{ $t("view_view.msg.inaccurate_only_building.btn") }}
        </button>
      </i>
    </div>
  </Teleport>

  <div class="toast toast-primary location-picker mb-2" v-if="marker2">
    <div class="columns">
      <div class="column col col-sm-12">
        {{ $t("view_view.msg.correct_location.msg") }}
      </div>
      <div class="column col-auto col-sm-12 btns">
        <button class="btn btn-sm" @click="cancelLocationPicker">
          {{ $t("view_view.msg.correct_location.btn-cancel") }}
        </button>
        <button class="btn btn-sm" @click="confirmLocationPicker">
          <i class="icon icon-check" />
          {{ $t("view_view.msg.correct_location.btn-done") }}
        </button>
      </div>
    </div>
  </div>
</template>
<style lang="scss"></style>
