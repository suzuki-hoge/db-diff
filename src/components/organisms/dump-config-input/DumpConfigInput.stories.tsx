import type { Meta, StoryObj } from '@storybook/react'

import { DumpConfigInput } from './DumpConfigInput'

const meta = {
  title: 'Organisms/DumpConfigInput',
  component: DumpConfigInput,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof DumpConfigInput>

export default meta
type Story = StoryObj<typeof meta>

export const InputIgnore: Story = {
  args: {
    tableName: 'users',
    colNames: ['id', 'name', 'age', 'created_at', 'updated_at'],
    value: 'ignore',
    input: true,
    onChange: console.log,
  },
}

export const InputOrdered: Story = {
  args: {
    tableName: 'users',
    colNames: ['id', 'name', 'age', 'created_at', 'updated_at'],
    value: 'updated_at',
    input: true,
    onChange: console.log,
  },
}

export const ViewOrdered: Story = {
  args: {
    tableName: 'users',
    colNames: ['id', 'name', 'age', 'created_at', 'updated_at'],
    value: 'updated_at',
    input: false,
    onChange: console.log,
  },
}
