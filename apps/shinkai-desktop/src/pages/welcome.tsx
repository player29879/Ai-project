import { useTranslation } from '@shinkai_network/shinkai-i18n';
import { buttonVariants, Checkbox } from '@shinkai_network/shinkai-ui';
import { cn } from '@shinkai_network/shinkai-ui/utils';
import { Link } from 'react-router-dom';

import { useSettings } from '../store/settings';
import OnboardingLayout from './layout/onboarding-layout';

const TermsAndConditionsPage = () => {
  const { t, Trans } = useTranslation();
  const termsAndConditionsAccepted = useSettings(
    (state) => state.termsAndConditionsAccepted,
  );
  const setTermsAndConditionsAccepted = useSettings(
    (state) => state.setTermsAndConditionsAccepted,
  );

  return (
    <OnboardingLayout>
      <div className="flex h-full flex-col justify-between">
        <p className="text-center text-3xl font-medium leading-[1.5] tracking-wide">
          {t('desktop.welcome')} <span aria-hidden> 🔑</span>
        </p>
        <div className="">
          <div className="flex flex-col gap-10">
            <div className="flex gap-3">
              <Checkbox
                checked={termsAndConditionsAccepted}
                id="terms"
                onCheckedChange={setTermsAndConditionsAccepted}
              />
              <label
                className="inline-block cursor-pointer text-xs leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                htmlFor="terms"
              >
                <span className={'leading-4 tracking-wide'}>
                  <Trans
                    components={{
                      a: (
                        <a
                          className={'text-white underline'}
                          href={'https://www.shinkai.com/terms-of-service'}
                          rel="noreferrer"
                          target={'_blank'}
                        />
                      ),
                      b: (
                        <a
                          className={'text-white underline'}
                          href={'https://www.shinkai.com/privacy-policy'}
                          rel="noreferrer"
                          target={'_blank'}
                        />
                      ),
                    }}
                    i18nKey="common.termsAndConditionsText"
                  />
                </span>
              </label>
            </div>
            <Link
              className={cn(
                buttonVariants({
                  variant: 'default',
                }),
                !termsAndConditionsAccepted &&
                  'pointer-events-none bg-gray-300 opacity-60',
              )}
              to={'/analytics'}
            >
              {t('common.getStarted')}
            </Link>
          </div>
        </div>
      </div>
    </OnboardingLayout>
  );
};

export default TermsAndConditionsPage;
