import type { Meta, StoryObj } from '@storybook/react'

import { SnapshotList } from './SnapshotList'
import { withRouter } from 'storybook-addon-react-router-v6'

const meta = {
  title: 'Templates/SnapshotList',
  component: SnapshotList,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof SnapshotList>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    snapshotSummaries: [
      {
        snapshotId: 'CC181CDB-07BC-4747-AABC-653CEF526D77',
        snapshotName: '初期状態',
        createAt: '2023/01/01 12:34:56',
      },
      {
        snapshotId: 'A650B18F-ADC6-462D-A619-D13F2D01CDD7',
        snapshotName: 'サインアップ ( Google アカウント連携 )',
        createAt: '2023/02/02 12:34:56',
      },
      {
        snapshotId: '77428FAF-A06E-4273-BF21-DE576CC35F43',
        snapshotName: '退会予約',
        createAt: '2023/03/03 12:34:56',
      },
      {
        snapshotId: '515D47B9-0744-4519-9C54-8E67F79687D3',
        snapshotName: '退会確定',
        createAt: '2023/04/04 12:34:56',
      },
    ],
    remove: console.log,
  },
}

export const Empty: Story = {
  args: {
    snapshotSummaries: [],
    remove: console.log,
  },
}
