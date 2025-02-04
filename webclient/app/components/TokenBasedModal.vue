<script setup lang="ts">
import { useFeedbackToken } from "~/composables/feedbackToken";
import { useFeedback } from "~/composables/feedback";

const props = defineProps<{
  data: { [index: string]: string | boolean | number };
}>();

const runtimeConfig = useRuntimeConfig();
const { t } = useI18n({ useScope: "local" });
const loading = ref(false);
const successUrl = ref("");
const { error, token } = useFeedbackToken(t);
const privacyChecked = ref(false);
const feedback = useFeedback();

function closeForm() {
  feedback.value.open = false;
  successUrl.value = "";
  error.value.blockSend = false;
  error.value.message = "";
}

enum SubmissionStatus {
  SUCCESSFULLY_CREATED = 201,
  UNAVAILABLE_FOR_LEGAL_REASONS = 451,
  SERVER_ERROR = 500,
  FORBIDDEN = 403,
}

function _send() {
  // data is a `Window` which cannot be cloned by `structuredClone`, but can be by JSON.
  const data = JSON.parse(JSON.stringify(props.data));
  data.privacy_checked = privacyChecked.value;
  data.token = token.value?.token;
  fetch(`${runtimeConfig.public.feedbackURL}/api/feedback/feedback`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
    .then((r) => {
      loading.value = false;
      if (r.status === SubmissionStatus.SUCCESSFULLY_CREATED) {
        token.value = null;
        r.text().then((url) => (successUrl.value = url));
      } else if (r.status === SubmissionStatus.SERVER_ERROR) {
        error.value.message = `${t("status.server_error")} (${r.text()})`;
      } else if (r.status === SubmissionStatus.UNAVAILABLE_FOR_LEGAL_REASONS) {
        error.value.message = t("error.please_accept_privacy_statement");
      } else if (r.status === SubmissionStatus.FORBIDDEN) {
        token.value = null;
        error.value.message = `${t("error.send_invalid_token")} (${r.text()})`;
      } else {
        // we reset the token here to be sure that it is the cause of the error
        token.value = null;
        error.value.message = `${t("status.send_unexpected_status")}: ${r.status}`;
      }
      if (r.status !== SubmissionStatus.SUCCESSFULLY_CREATED) {
        document.getElementById("token-modal-error")?.scrollIntoView({ behavior: "smooth" });
      }
    })
    .catch((r) => {
      loading.value = false;
      error.value.message = t("error.send_req_failed");
      console.error(r);
      document.getElementById("token-modal-error")?.scrollIntoView({ behavior: "smooth" });
    });
}

function sendForm() {
  // validate the own form
  if (token.value === null) {
    error.value.message = t("error.send_no_token");
    error.value.blockSend = true;
    return;
  }
  if (!privacyChecked.value) {
    error.value.message = t("error.please_accept_privacy_statement");
    return;
  }

  // validate the foreign form
  if (feedback.value.data.subject.length < 3) {
    error.value.message = t("error.form.too_short_subject");
    return;
  }
  if (feedback.value.data.body.length < 10) {
    error.value.message = t("error.form.too_short_body");
    return;
  }

  loading.value = true;
  // Token may only be used after a short delay.
  const MINIMUM_DELAY_MS = 10_000;
  const timeSinceTokenCreationInMs = Date.now() - token.value.created_at;
  if (timeSinceTokenCreationInMs < MINIMUM_DELAY_MS) setTimeout(_send, MINIMUM_DELAY_MS - timeSinceTokenCreationInMs);
  else _send();
}
</script>

<template>
  <Modal v-if="!successUrl" v-model="feedback.open" :title="t('title')" @close="closeForm">
    <Toast v-if="error.message" class="mb-4" :msg="error.message" level="error" />

    <div class="flex flex-col gap-1">
      <slot name="modal" />
      <div>
        <Checkbox id="privacy-checked" v-model="privacyChecked">
          <template #default>
            <I18nT tag="p" keypath="public.agreement" class="font-bold">
              <template #github_project_issues>
                <NuxtLink
                  tabindex="1"
                  class="text-blue-600 visited:text-blue-600 hover:underline"
                  to="https://github.com/TUM-Dev/navigatum/issues"
                  target="_blank"
                  external
                >
                  {{ t("public.github_project_issues") }}
                </NuxtLink>
              </template>
            </I18nT>
          </template>
          <template #helptext>
            <p>
              <I18nT tag="span" keypath="public.disclaimer">
                <template #github_site_policy>
                  <NuxtLink
                    tabindex="1"
                    class="text-blue-600 visited:text-blue-600 hover:underline"
                    to="https://docs.github.com/en/github/site-policy"
                    target="_blank"
                    external
                  >
                    {{ t("public.github_site_policy") }}
                  </NuxtLink>
                </template>
              </I18nT>
              {{ t("public.processing_based_on_gdpr") }}
            </p>
            <p>
              {{ t("public.right_to_information") }}
              {{ t("public.right_of_appeal") }}
            </p>
            <p>
              <I18nT keypath="public.objection_instruction" tag="span">
                <template #imprint>
                  <NuxtLinkLocale
                    tabindex="1"
                    to="/about/impressum"
                    class="text-blue-600 visited:text-blue-600 hover:underline"
                  >
                    {{ t("public.imprint") }}
                  </NuxtLinkLocale>
                </template>
              </I18nT>
              <br />
              <I18nT keypath="public.question_contact">
                <template #datenschutz>
                  <NuxtLink
                    tabindex="1"
                    class="text-blue-600 visited:text-blue-600 hover:underline"
                    to="https://datenschutz.tum.de"
                    target="_blank"
                    external
                    >datenschutz.tum.de
                  </NuxtLink>
                </template>
              </I18nT>
            </p>
          </template>
        </Checkbox>
      </div>
    </div>

    <div class="float-right flex flex-row-reverse gap-2">
      <Btn
        variant="primary"
        size="md"
        :class="{
          'cursor-progress !bg-blue-200 !text-blue-900': loading,
          'cursor-not-allowed !bg-blue-300 !text-blue-50': error.blockSend,
        }"
        v-bind="{ disabled: loading || error.blockSend }"
        @click="sendForm"
      >
        <template v-if="loading">
          <Spinner class="my-auto h-4 w-4" />
          {{ t("sending...") }}
        </template>
        <template v-else-if="error.blockSend">{{ t("try_again_later") }}</template>
        <template v-else>{{ t("send") }}</template>
      </Btn>
      <Btn variant="linkButton" size="md" @click="closeForm">
        {{ t("cancel") }}
      </Btn>
    </div>
  </Modal>
  <Modal v-if="successUrl" v-model="feedback.open" :title="t('thank_you')" @close="closeForm">
    <slot name="success" :success-url="successUrl" />

    <Btn size="md" variant="primary" @click="closeForm">OK</Btn>
  </Modal>
</template>

<i18n lang="yaml">
de:
  title: Feedback senden
  cancel: Abbrechen
  error:
    token_unexpected_status: "Unerwarteter Status Code beim Abrufen eines Feedback Tokens: "
    token_req_failed: Unerwarteter Fehler beim Laden des Feedback-Formulars. Das Senden von Feedback ist gerade vermutlich nicht möglich. Bitte schreibe stattdessen eine Mail.
    too_many_requests: Feedback senden ist aktuell nicht möglich aufgrund von rate-limiting. Bitte versuche es später nochmal oder schreibe eine Mail.
    send_invalid_token: Formular-Token ungültig (vermutlich abgelaufen). Bitte kopiere den Text und öffne das Formular nochmal.
    please_accept_privacy_statement: Du musst die Datenschutzerklärung akzeptiert haben, damit wir dein Feedback via GitHub verarbeiten können.
    feedback_not_configured: Das Senden von Feedback ist auf dem Server aktuell nicht konfiguriert.
    send_no_token: Ein unerwarteter Fehler ist aufgetreten (Kein Token). Bitte kopiere den Text und öffne das Formular nochmal.
    send_req_failed: Unerwarteter Fehler beim Senden des Feedback-Formulars. Das Senden von Feedback ist gerade vermutlich nicht möglich. Bitte schreibe stattdessen eine Mail.
    form:
      too_short_body: "Fehler: Nachricht fehlt oder ist zu kurz"
      too_short_subject: "Fehler: Betreff fehlt oder ist zu kurz"
  status:
    send_unexpected_status: Unerwarteter Status Code
    server_error: Server Fehler
  public:
    agreement: Meine Feedback Daten (Betreff und Nachricht) dürfen anonym, aber öffentlich zugänglich auf der {github_project_issues} gespeichert werden.
    disclaimer: Mit der Nutzung dieses Feedbackformulars stimmst du explizit den {github_site_policy} sowie einer möglichen Übertragung der Daten außerhalb der Europäischen Union zu.
    github_project_issues: GitHub Projektseite
    github_site_policy: Nutzungsbedingungen und Datenschutzbestimmungen von GitHub
    imprint: Impressum gelisteten Kontaktmöglichkeiten
    objection_instruction: Falls du dies ablehnst, schreibe uns bitte über navigatum (at-symbol) tum.de, oder eine der anderen Optionen aus unserem {imprint}.
    processing_based_on_gdpr: Die Verarbeitung basiert auf Grundlage des Art. 6 Abs.1 lit. a DSGVO.
    question_contact: Bei Fragen kannst du dich gerne an uns (navigatum (at-symbol) tum.de) oder an unseren Datenschutzbeauftragten ({datenschutz}) wenden.
    right_of_appeal: Es besteht zudem ein Beschwerderecht beim Bayerischen Landesbeauftragten für den Datenschutz.
    right_to_information: Unter den gesetzlichen Voraussetzungen und einem vorhandenen Personenbezug der Daten besteht ein Recht auf Auskunft, sowie auf Berichtigung oder Löschung oder auf Einschränkung der Verarbeitung oder eines Widerspruchsrechts gegen die Verarbeitung sowie des Rechts auf Datenübertragbarkeit.
  sending: Wird gesendet
  try_again_later: Bitte versuche es später noch einmal
  send: Senden
  thank_you: Vielen Dank!
en:
  title: Send Feedback
  cancel: Cancel
  error:
    token_unexpected_status: "Unexpected status code when retrieving a feedback token: "
    send_invalid_token: Invalid form token (probably expired). Please copy the text and re-open the form.
    please_accept_privacy_statement: You have to accept the privacy statement for us to process the feedback via GitHub.
    feedback_not_configured: Sending feedback is currently not configured on the server.
    send_no_token: An unexpected error occured (no token). Please copy the text and re-open the form.
    send_req_failed: Unexpected error when sending the feedback form. Sending feedback is currently probably not possible. Please send a mail instead.
    token_req_failed: Unexpected error when loading the feedback form. Sending feedback is currently probably not possible. Please send a mail instead.
    too_many_requests: Sending feedback is currently not possible due to rate-limiting. Please try again in a while or send a mail.
    form:
      too_short_body: "Error: Message missing or too short"
      too_short_subject: "Error: Subject missing or too short"
  status:
    server_error: Server Error
    send_unexpected_status: Unexpected status code
  public:
    agreement: My feedback data (subject and message) may be stored anonymously but publicly accessible on the {github_project_issues}.
    disclaimer: By using this feedback form, you explicitly agree to the {github_site_policy} as well as a possible transfer of the data outside the European Union.
    github_project_issues: GitHub project page
    github_site_policy: terms of use and privacy policy of GitHub
    imprint: contact options listed in our imprint
    objection_instruction: If you object to this, please write to us via navigatum (at-symbol) tum.de, or one of the other other options from the {imprint}.
    processing_based_on_gdpr: The processing is based on Art. 6 para. 1 lit. a DSGVO.
    question_contact: If you have any questions, please feel free to contact us (navigatum (at-symbol) tum.de) or our data protection officer ({datenschutz}).
    right_of_appeal: There is also a right of appeal to the Bavarian State Commissioner for Data Protection.
    right_to_information: Under the legal conditions and an existing personal reference of the data, there is a right to information, as well as to correction or deletion or to restriction of processing or a right to object to processing as well as the right to data portability.
  sending: Sending
  try_again_later: not possible
  send: Send
  thank_you: Thank you!
</i18n>
