import { LocaleMode, switchLanguage } from '@shinkai_network/shinkai-i18n';
import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';

type SettingsStore = {
  defaultAgentId: string;
  setDefaultAgentId: (defaultAgentId: string) => void;
  sidebarExpanded: boolean;
  toggleSidebar: () => void;
  isGetStartedChecklistHidden: boolean;
  setGetStartedChecklistHidden: (isGetStartedChecklistHidden: boolean) => void;
  termsAndConditionsAccepted?: boolean;
  setTermsAndConditionsAccepted: (termsAndConditionsAccepted: boolean) => void;
  optInAnalytics?: boolean;
  acceptAnalytics: () => void;
  denyAnalytics: () => void;
  optInExperimental?: boolean;
  setOptInExperimental: (optInExperimental: boolean) => void;
  userLanguage: LocaleMode;
  setUserLanguage: (userLanguage: LocaleMode) => void;
  evmAddress: string;
  setEvmAddress: (evmAddress: string) => void;
};

export const useSettings = create<SettingsStore>()(
  devtools(
    persist(
      (set) => ({
        defaultAgentId: '',
        setDefaultAgentId: (defaultAgentId) => {
          set({ defaultAgentId });
        },

        sidebarExpanded: false,
        toggleSidebar: () => {
          set((state) => ({ sidebarExpanded: !state.sidebarExpanded }));
        },

        isGetStartedChecklistHidden: false,
        setGetStartedChecklistHidden: (isGetStartedChecklistHidden) => {
          set({ isGetStartedChecklistHidden });
        },

        termsAndConditionsAccepted: undefined,
        setTermsAndConditionsAccepted: (termsAndConditionsAccepted) => {
          set({ termsAndConditionsAccepted });
        },

        optInAnalytics: undefined,
        acceptAnalytics: () => {
          set({ optInAnalytics: true });
        },
        denyAnalytics: () => {
          set({ optInAnalytics: false });
        },

        optInExperimental: false,
        setOptInExperimental: (optInExperimental) => {
          set({ optInExperimental });
        },

        userLanguage: 'auto',
        setUserLanguage: (userLanguage) => {
          set({ userLanguage });
          switchLanguage(userLanguage);
        },

        evmAddress: '',
        setEvmAddress: (evmAddress) => {
          set({ evmAddress });
        },
      }),
      {
        name: 'settings',
      },
    ),
  ),
);
