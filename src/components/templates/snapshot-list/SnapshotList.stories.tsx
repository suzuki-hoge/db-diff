import type { Meta, StoryObj } from '@storybook/react'

import { SnapshotList } from './SnapshotList'

const meta = {
  title: 'Templates/SnapshotList',
  component: SnapshotList,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof SnapshotList>

export default meta
type Story = StoryObj<typeof meta>
export const Component: Story = {
  args: {
    snapshotSummaries: [
      {
        id: 'CC181CDB-07BC-4747-AABC-653CEF526D77',
        name: '初期状態',
        createAt: '2023/01/01 12:34:56',
      },
      {
        id: 'A650B18F-ADC6-462D-A619-D13F2D01CDD7',
        name: 'サインアップ ( Google アカウント連携 )',
        createAt: '2023/02/02 12:34:56',
      },
      {
        id: '77428FAF-A06E-4273-BF21-DE576CC35F43',
        name: '退会予約',
        createAt: '2023/03/03 12:34:56',
      },
      {
        id: '515D47B9-0744-4519-9C54-8E67F79687D3',
        name: '退会確定',
        createAt: '2023/04/04 12:34:56',
      },
    ],
  },
}
