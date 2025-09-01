// GDPR Consent Management System
// Handles analytics consent, PostHog initialization, and privacy controls

// Consent Management Functions
function hasConsent() {
    const consent = localStorage.getItem('analytics-consent');
    const consentDate = localStorage.getItem('analytics-consent-date');

    if (!consent || !consentDate) return false;

    // Check if consent is older than 12 months
    const consentTime = new Date(consentDate).getTime();
    const oneYearAgo = new Date().getTime() - (365 * 24 * 60 * 60 * 1000);

    if (consentTime < oneYearAgo) {
        localStorage.removeItem('analytics-consent');
        localStorage.removeItem('analytics-consent-date');
        return false;
    }

    return consent === 'true';
}

function setConsent(accepted) {
    localStorage.setItem('analytics-consent', accepted.toString());
    localStorage.setItem('analytics-consent-date', new Date().toISOString());

    if (accepted) {
        initializePostHog();
    }

    hideConsentBanner();
}

// Consent Banner Management
function showConsentBanner() {
    document.getElementById('consentBanner').classList.remove('hidden');
}

function hideConsentBanner() {
    document.getElementById('consentBanner').classList.add('hidden');
}

// PostHog Analytics Integration
function initializePostHog() {
    !function (t, e) { var o, n, p, r; e.__SV || (window.posthog = e, e._i = [], e.init = function (i, s, a) { function g(t, e) { var o = e.split("."); 2 == o.length && (t = t[o[0]], e = o[1]), t[e] = function () { t.push([e].concat(Array.prototype.slice.call(arguments, 0))) } } (p = t.createElement("script")).type = "text/javascript", p.crossOrigin = "anonymous", p.async = !0, p.src = s.api_host.replace(".i.posthog.com", "-assets.i.posthog.com") + "/static/array.js", (r = t.getElementsByTagName("script")[0]).parentNode.insertBefore(p, r); var u = e; for (void 0 !== a ? u = e[a] = [] : a = "posthog", u.people = u.people || [], u.toString = function (t) { var e = "posthog"; return "posthog" !== a && (e += "." + a), t || (e += " (stub)"), e }, u.people.toString = function () { return u.toString(1) + ".people (stub)" }, o = "init Ie Ts Ms Ee Es Rs capture Ge calculateEventProperties Os register register_once register_for_session unregister unregister_for_session js getFeatureFlag getFeatureFlagPayload isFeatureEnabled reloadFeatureFlags updateEarlyAccessFeatureEnrollment getEarlyAccessFeatures on onFeatureFlags onSurveysLoaded onSessionId getSurveys getActiveMatchingSurveys renderSurvey canRenderSurvey canRenderSurveyAsync identify setPersonProperties group resetGroups setPersonPropertiesForFlags resetPersonPropertiesForFlags setGroupPropertiesForFlags resetGroupPropertiesForFlags reset get_distinct_id getGroups get_session_id get_session_replay_url alias set_config startSessionRecording stopSessionRecording sessionRecordingStarted captureException loadToolbar get_property getSessionProperty Ds Fs createPersonProfile Ls Ps opt_in_capturing opt_out_capturing has_opted_in_capturing has_opted_out_capturing clear_opt_in_out_capturing Cs debug I As getPageViewId captureTraceFeedback captureTraceMetric".split(" "), n = 0; n < o.length; n++)g(u, o[n]); e._i.push([i, s, a]) }, e.__SV = 1) }(document, window.posthog || []);
    
    // Updated PostHog configuration with modern privacy settings
    posthog.init('phc_oPXS4I08n2TakqavgbpWkH16iME2iE2j94eTzpaj4tp', {
        api_host: 'https://eu.i.posthog.com',
        person_profiles: 'identified_only',
        disable_session_recording: true,
        respect_dnt: true,
        capture_pageview: true,
        capture_pageleave: false
    });
}

// Privacy Settings Modal Management
function showPrivacyModal() {
    const modal = document.getElementById('privacyModal');
    updatePrivacyModalContent();
    modal.classList.remove('hidden');
    modal.classList.add('flex');
}

function hidePrivacyModal() {
    const modal = document.getElementById('privacyModal');
    modal.classList.add('hidden');
    modal.classList.remove('flex');
}

function updatePrivacyModalContent() {
    const statusText = document.getElementById('consentStatusText');
    const toggleBtn = document.getElementById('toggleBtnText');
    const currentConsent = hasConsent();

    if (currentConsent) {
        statusText.textContent = 'Enabled';
        toggleBtn.textContent = 'Disable';
    } else {
        statusText.textContent = 'Disabled';
        toggleBtn.textContent = 'Enable';
    }
}

function toggleConsent() {
    const currentConsent = hasConsent();
    const newConsent = !currentConsent;

    setConsent(newConsent);
    updatePrivacyModalContent();

    // Show user feedback
    const statusText = document.getElementById('consentStatusText');
    statusText.textContent = newConsent ? 'Enabled' : 'Disabled';
}

// Initialize consent system when page loads
document.addEventListener('DOMContentLoaded', function () {
    // Check consent and initialize PostHog if consented
    if (hasConsent()) {
        initializePostHog();
    } else {
        showConsentBanner();
    }

    // Set up consent banner event listeners
    document.getElementById('acceptConsent').addEventListener('click', function () {
        setConsent(true);
    });

    document.getElementById('rejectConsent').addEventListener('click', function () {
        setConsent(false);
    });

    // Set up privacy modal event listeners
    document.getElementById('privacySettingsLink').addEventListener('click', function (e) {
        e.preventDefault();
        showPrivacyModal();
    });

    document.getElementById('closePrivacyModal').addEventListener('click', function () {
        hidePrivacyModal();
    });

    document.getElementById('toggleConsentBtn').addEventListener('click', function () {
        toggleConsent();
    });

    // Close modal when clicking outside
    document.getElementById('privacyModal').addEventListener('click', function (e) {
        if (e.target === this) {
            hidePrivacyModal();
        }
    });
});