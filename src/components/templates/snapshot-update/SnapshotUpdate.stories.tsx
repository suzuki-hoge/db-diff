import type { Meta, StoryObj } from '@storybook/react'

import { SnapshotUpdate } from './SnapshotUpdate'
import { withRouter } from 'storybook-addon-react-router-v6'

const meta = {
  title: 'Templates/SnapshotUpdate',
  component: SnapshotUpdate,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof SnapshotUpdate>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    snapshotSummary: {
      snapshotId: 'CC181CDB-07BC-4747-AABC-653CEF526D77',
      snapshotName: '初期状態',
      createAt: '2023/01/01 12:34:56',
    },
    dumpConfigs: [
      { tableName: 'groups', colNames: ['id', 'name', 'created_at', 'updated_at'], value: 'limited' },
      { tableName: 'users', colNames: ['id', 'name', 'created_at', 'updated_at'], value: 'updated_at' },
    ],
    update: console.log,
  },
}
