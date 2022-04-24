import { useIntl } from 'umi';
import { GithubOutlined } from '@ant-design/icons';
import { DefaultFooter } from '@ant-design/pro-layout';

const Footer: React.FC = () => {
  const intl = useIntl();
  const defaultMessage = intl.formatMessage({
    id: 'app.copyright.produced',
    defaultMessage: 'James Zow Apache-2.0 License',
  });

  const currentYear = new Date().getFullYear();

  return (
    <DefaultFooter
      copyright={`${currentYear} ${defaultMessage}`}
      links={[
        {
          key: 'James Zow',
          title: 'James Zow',
          href: 'https://github.com/Jzow/',
          blankTarget: true,
        },
        {
          key: 'github',
          title: <GithubOutlined />,
          href: 'https://github.com/Jzow/rustlibrary',
          blankTarget: true,
        },
        {
          key: 'Apache-2.0 License',
          title: 'Apache-2.0 License',
          href: 'https://github.com/Jzow/rustlibrary/blob/master/LICENSE',
          blankTarget: true,
        },
      ]}
    />
  );
};

export default Footer;
