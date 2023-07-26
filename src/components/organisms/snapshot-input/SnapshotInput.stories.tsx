import type { Meta, StoryObj } from '@storybook/react'

import { SnapshotInput } from './SnapshotInput'

const meta = {
  title: 'Organisms/SnapshotInput',
  component: SnapshotInput,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof SnapshotInput>

export default meta
type Story = StoryObj<typeof meta>

export const Create: Story = {
  args: {
    dump: console.log,
  },
}

export const Update: Story = {
  args: {
    snapshotSummary: {
      snapshotId: 'CC181CDB-07BC-4747-AABC-653CEF526D77',
      snapshotName: '初期状態',
      createAt: '2023/01/01 12:34:56',
    },
    update: console.log,
  },
}
