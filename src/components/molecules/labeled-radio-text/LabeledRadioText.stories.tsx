import type { Meta, StoryObj } from '@storybook/react'

import { LabeledRadioText } from './LabeledRadioText'

const meta = {
  title: 'Molecules/LabeledRadioText',
  component: LabeledRadioText,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof LabeledRadioText>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    label: 'System',
    value: 'MySQL',
    values: ['MySQL', 'PostgreSQL'],
    name: 'system',
    onChange: console.log,
  },
}
