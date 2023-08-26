import type { Meta, StoryObj } from '@storybook/react'

import { SnapshotCreate } from './SnapshotCreate'
import { withRouter } from 'storybook-addon-react-router-v6'

const meta = {
  title: 'Templates/SnapshotCreate',
  component: SnapshotCreate,
  tags: ['autodocs'],
  argTypes: {},
  decorators: [withRouter],
} satisfies Meta<typeof SnapshotCreate>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    dumpConfigs: [
      { tableName: 'groups', colNames: ['id', 'name', 'created_at', 'updated_at'], value: 'limited' },
      { tableName: 'users', colNames: ['id', 'name', 'created_at', 'updated_at'], value: 'updated_at' },
    ],
    dump: console.log,
  },
}
